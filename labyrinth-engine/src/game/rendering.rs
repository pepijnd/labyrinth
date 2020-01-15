use std::ops::Deref;

use glium::{
    backend::{Context, Facade},
    Surface,
};

use crate::game::camera::Camera;
use crate::game::context::SharedContext;
use crate::game::counter::Counter;
use crate::game::Game;

use labyrinth_cgmath::prelude::*;
use labyrinth_cgmath::Rad;
use labyrinth_cgmath::{FloatMat4, FloatPoint3, FloatVec3};

use crate::resources::{
    Findable,
    material::EffectBuffer,
    material::EffectUniform,
    shader::ProgramBuffer,
    object::ObjectBuffer,
    model::MeshBuffer,
    animation::SkeletonBuffer,
    animation::AnimationBuffer,
};

use crate::game::entity::Entity;

use crate::labyrinth_error;

use labyrinth_glyph::glyph_brush::rusttype::Font;
use labyrinth_glyph::GlyphBrush;

use glyph_brush::{SectionText, VariedSection};

use generational_arena::Index;

#[derive(Debug)]
enum RenderError {
    Render(Box<dyn std::error::Error>)
}

labyrinth_error!(RenderError, |e| match e {
    RenderError::Render(e) => {
        format!("{}", e)
    }
});

#[derive(Copy, Clone)]
struct DebugVertex {
    pos: [f32; 2],
    tex: [f32; 2],
}

impl DebugVertex {
    fn new(pos: [f32; 2], tex: [f32; 2]) -> DebugVertex {
        DebugVertex { pos, tex }
    }
}

implement_vertex!(DebugVertex, pos, tex);

fn make_debug<F>(facade: &F) -> Result<(glium::VertexBuffer<DebugVertex>, glium::IndexBuffer<u16>), RenderError>
where
    F: Facade + Deref<Target = Context>,
{
    let vertices = glium::VertexBuffer::new(
        facade,
        &[
            DebugVertex::new([-1.0, -1.0], [0.0, 0.0]),
            DebugVertex::new([1.0, -1.0], [1.0, 0.0]),
            DebugVertex::new([-1.0, 1.0], [0.0, 1.0]),
            DebugVertex::new([1.0, 1.0], [1.0, 1.0]),
        ],
    ).map_err(|e| RenderError::Render(e.into()))?;
    let indices = glium::IndexBuffer::new(
        facade,
        glium::index::PrimitiveType::TrianglesList,
        &[0, 1, 2, 1, 3, 2],
    ).map_err(|e| RenderError::Render(e.into()))?;
    Ok((vertices, indices))
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Light {
    position: FloatVec3,
    _pad1: [f32; 1],
    direction: FloatVec3,
    _pad2: [f32; 1],
    color: FloatVec3,
}

impl Light {
    fn new(position: FloatVec3, direction: FloatVec3, color: FloatVec3) -> Light {
        Light {
            position,
            _pad1: [0f32; 1],
            direction,
            _pad2: [0f32; 1],
            color,
        }
    }
}

implement_uniform_block!(Light, position, direction, color);

#[derive(Copy, Clone)]
pub struct LightMap {
    light: Light,
}

impl LightMap {
    fn new(light: Light) -> LightMap {
        LightMap { light }
    }
}

implement_uniform_block!(LightMap, light);

#[derive(Copy, Clone)]
pub struct MaterialMap {
    material: EffectUniform,
}

impl MaterialMap {
    pub fn new(material: &EffectBuffer) -> MaterialMap {
        MaterialMap {
            material: material.to_uniform(),
        }
    }
}

implement_uniform_block!(MaterialMap, material);

#[derive(Copy, Clone)]
pub struct BonesMap {
    bones: BoneUniform
}

implement_uniform_block!(BonesMap, bones);

#[derive(Copy, Clone)]
pub struct BoneUniform {
    bones: [FloatMat4; 10]
}

implement_uniform_block!(BoneUniform, bones);

pub struct RenderCommand {
    pub matrix: FloatMat4,
    pub depth_mvp: FloatMat4,
    pub material: MaterialMap,
    pub mesh: Index,
    //pub texture: Index,
    pub program: Index,
}

impl RenderCommand {
    pub fn new(
        material: MaterialMap,
        mesh: Index,
        //texture: Index,
        program: Index,
    ) -> RenderCommand {
        RenderCommand {
            matrix: labyrinth_cgmath::num_traits::one(),
            depth_mvp: FloatMat4::identity(),
            material,
            mesh,
            //texture,
            program,
        }
    }
}

pub struct RenderBuffer {
    inner: Vec<RenderCommand>,
}

impl RenderBuffer {
    fn new() -> RenderBuffer {
        RenderBuffer { inner: Vec::new() }
    }

    pub fn sort(&mut self) {
        self.inner.sort_by(|a, b| {
            b.material
                .material
                .alpha
                .partial_cmp(&a.material.material.alpha)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn push(&mut self, command: RenderCommand) {
        self.inner.push(command);
    }
}

pub struct Renderer<'a> {
    pub counter: Counter,
    font: Option<GlyphBrush<'a, 'a>>,
    shadow_map: Option<glium::texture::DepthTexture2d>,
}

impl<'a> Renderer<'a> {
    pub fn new() -> Renderer<'a> {
        Renderer {
            counter: Counter::new(),
            font: None,
            shadow_map: None,
        }
    }

    pub fn render<T, F>(&mut self, game: &Game, target: &mut T, facade: &F, context: SharedContext) -> crate::LabyrinthResult<()>
    where
        T: Surface,
        F: Facade + Deref<Target = Context>,
    {
        match &self.font {
            Some(_) => {},
            None => {
                let font_src: &[u8] = include_bytes!("/home/pepijn/Projects/labyrinth/assets/ConnectionII.ttf");
                let font = GlyphBrush::new(facade, vec![Font::from_bytes(font_src).map_err(|e| RenderError::Render(e.into()))?]);
                self.font = Some(font);
            }
        }
        let brush = self.font.as_mut().unwrap();

        {
            let mut context = context.borrow_mut();
            context.t += 0.0075;
        }

        match &self.shadow_map {
            Some(_) => {},
            None => {
                let map = glium::texture::DepthTexture2d::empty(facade, 512, 512).map_err(|e| RenderError::Render(e.into()))?;
                self.shadow_map = Some(map);
            }
        }
        let shadow_map = self.shadow_map.as_ref().unwrap();

        let mut buffer = RenderBuffer::new();

        let shared = context.clone();
        let context = shared.borrow();

        let player = ObjectBuffer::find(&context, "BoxChar")?;
        let player = Entity::new("BoxChar", player);

        let skeleton = SkeletonBuffer::find(&context, "BoxChar")?;
        let _skeleton = SkeletonBuffer::get(&context, skeleton);

        let animation = AnimationBuffer::find(&context, "BoxChar")?;
        let _animation = AnimationBuffer::get(&context, animation)?;

        player.render_queue(&context, &mut buffer)?;

        for entity in game.entities.iter() {
            entity.render_queue(&context, &mut buffer)?;
        }
        buffer.sort();

        let mut camera = Camera::new();
        *camera.get_position_mut() = FloatPoint3::new(3.5, 2.5, -3.5);
        *camera.get_look_at_mut() = FloatPoint3::new(0.0, 1.0, 0.0);

        let pos = FloatVec3::new((context.t * 1.75).sin() * 4.0 + 2.0, 3.5, context.t.cos() * 8.0);

        let light = Light::new(
            pos,
            (-pos).normalize(),
            FloatVec3::new(1.0, 1.0, 1.0),
        );
        let lightmap = glium::uniforms::UniformBuffer::new(facade, LightMap::new(light))
            .map_err(|e| RenderError::Render(e.into()))?;

        let _w = 4.0;
        let dpm = labyrinth_cgmath::perspective(Rad::full_turn() / 8.0, 1.0, 2.0, 50.0);
        let dvm = labyrinth_cgmath::Matrix4::look_at(
            FloatPoint3::from_vec(light.position),
            FloatPoint3::from_vec(light.position + light.direction),
            FloatVec3::new(0.0, 1.0, 0.0),
        );

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let mut shadow_target =
            glium::framebuffer::SimpleFrameBuffer::depth_only(facade, &*shadow_map).map_err(|e| RenderError::Render(e.into()))?;
        shadow_target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

        for command in buffer.inner.iter_mut() {
            let depth_mvp = dpm * dvm * command.matrix;
            command.depth_mvp = depth_mvp;

            let mesh = MeshBuffer::get(&context, command.mesh)?;
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
            let program = ProgramBuffer::find(&context, "shadow")?;
            let program = ProgramBuffer::get(&context, program)?;

            let uniforms = uniform!(depth_mvp: depth_mvp,);

            shadow_target
                .draw(
                    &mesh.vertices,
                    &indices,
                    &program.program,
                    &uniforms,
                    &params,
                )
                .map_err(|e| RenderError::Render(e.into()))?;
        }

        let bias_matrix: FloatMat4 = [
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.5, 0.5, 0.5, 1.0],
        ]
        .into();

        let perspective: FloatMat4 = {
            let (width, height) = target.get_dimensions();
            let ratio = width as f32 / height as f32;
            let fovx: Rad<f32> = labyrinth_cgmath::Deg(90f32).into();
            let fovy = Rad::atan((fovx/2.0).tan() * (1.0 / ratio)) * 2.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let perspective = labyrinth_cgmath::PerspectiveFov {
                fovy: fovy,
                aspect: ratio,
                near: znear,
                far: zfar,
            };
            perspective.into()
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            smooth: None,
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };

        let bones =  glium::uniforms::UniformBuffer::new(facade, 
            BonesMap { bones: BoneUniform { bones: [labyrinth_cgmath::One::one(); 10] }}).unwrap();

        for command in buffer.inner.iter() {
            let material = glium::uniforms::UniformBuffer::new(facade, command.material).map_err(|e| RenderError::Render(e.into()))?;
            //let tex = context.get_texture(&command.texture).unwrap();
            //let tex = tex.borrow();
            let depth_bias_mvp = bias_matrix * command.depth_mvp;
            //let basetex = tex.basetexture.borrow();
            //let normal = tex.normal.borrow();
            let mesh = MeshBuffer::get(&context, command.mesh)?;
            let matrix = command.matrix;
            let program = ProgramBuffer::get(&context, command.program)?;
            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

            let uniforms = uniform! {
                perspective: perspective,
                matrix: matrix,
                depth_bias_mvp: depth_bias_mvp,
                //tex: &*basetex,
                shadow_map: glium::uniforms::Sampler::new(&*shadow_map)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual)),
                //normal_map: &*normal,
                view: camera.look_at(),
                camera_pos: camera.get_position(),
                matmap: &material,
                lightmap: &lightmap,
                bones: &bones,
            };
            target
                .draw(
                    &mesh.vertices,
                    &indices,
                    &program.program,
                    &uniforms,
                    &params,
                )
                .map_err(|e| RenderError::Render(e.into()))?;
        }

        let debug = make_debug(facade)?;
        let uniforms = uniform!(
            tex: glium::uniforms::Sampler::new(&*shadow_map)
                    .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
        );
        let program = ProgramBuffer::find(&context, "debug")?;
        let program = ProgramBuffer::get(&context, program)?;
        target.clear_depth(1.0);
        target
            .draw(
                &debug.0,
                &debug.1,
                &program.program,
                &uniforms,
                &Default::default(),
            )
            .map_err(|e| RenderError::Render(e.into()))?;

        let screen_dims = target.get_dimensions();
        brush.queue(VariedSection {
            text: vec![SectionText {
                text: format!("fps: {:.1}", self.counter.get_rate()).as_str(),
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            }],
            bounds: (screen_dims.0 as f32, screen_dims.1 as f32),
            ..Default::default()
        });
        brush.draw_queued(facade, target);

        self.counter.count(|| {});

        Ok(())
    }
}
