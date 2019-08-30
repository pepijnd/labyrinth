use std::rc::Rc;
use std::cell::RefCell;

use glium::{backend::Facade, Surface};

use crate::game::rendering::RenderTarget;
use crate::game::context::SharedContext;
use crate::resources::model::IndiceType;
use crate::resources::model::Vertex;
use num_traits::identities::One;
use crate::game::math::Uniform;
use crate::game::camera::Camera;
use crate::resources::material::Material;
use crate::resources::texture::Texture;
use crate::game::context::Shared;
use crate::resources::model::Model;


#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub model: Shared<Model>,
    //pub material: Shared<Material>
}

impl Object {
    pub fn new(name: String, model: Shared<Model>) -> Object {
        Object {
            name,
            model,
            //material
        }
    }
}

impl<T, F> RenderTarget<T, F> for Object
where
    T: Surface,
    F: Facade,
{
    fn render(&self, target: &mut T, facade: &F, context: SharedContext) {
        {
            let mut context = context.borrow_mut();
            context.t += 0.01;
                        
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                smooth: None,
                .. Default::default()
            };

            let light = [-1.0, 0.4, 0.9f32];
            let perspective = {
                let (width, height) = target.get_dimensions();
                let ratio = width as f32 / height as f32;
                let fov: f32 = 3.141592 / 3.0;
                let zfar = 1024.0;
                let znear = 0.1;
                
                glm::ext::perspective(fov, ratio, znear, zfar)
            };

            let mut camera = Camera::new();
            *camera.get_position_mut() = glm::vec3(context.t.sin()*6.0, 2.2, context.t.cos()*6.0);

            let vertex_shader_src = include_str!("../../../assets/vertex.glsl");
            let fragment_shader_src = include_str!("../../../assets/fragment.glsl");

            for mesh in self.model.borrow().meshes.iter() {
                let indices = match mesh.indices {
                    IndiceType::None(indices) => indices,
                    _ => glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
                };
                let material = mesh.material.as_ref().unwrap().borrow();
                let basematerial = material.basematerial.borrow();
                let texture = material.texture.as_ref().unwrap().borrow();
                let basetexture = &*texture.basetexture.borrow();

                let program = glium::Program::from_source(facade, vertex_shader_src, fragment_shader_src, None).unwrap();

                let uniforms = uniform! {
                    perspective: perspective.as_uniform(),
                    matrix: glm::ext::translate(&glm::Matrix4::one(), glm::vec3(0.0, -3.0, 0.0)).as_uniform(),
                    tex: basetexture,
                    view: camera.make_view(),
                    camera_pos: *camera.get_position().as_array(),
                    u_light: light
                };
                target.draw(
                    &mesh.buffer,
                    &indices,
                    &program,
                    &uniforms,
                    &params).unwrap();
            }
        }
    }
}
