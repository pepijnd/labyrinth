use std::rc::Rc;
use std::cell::RefCell;

use glium::backend::Facade;
use glium::index::IndicesSource;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::vertex::MultiVerticesSource;
use glium::IndexBuffer;
use glium::VertexBuffer;

use labyrinth_obj::obj::Primitive::Triangle;

use crate::resources::material::Material;
use crate::game::context::SharedContext;
use crate::game::context::LabyrinthContext;
use crate::game::context::Shared;

use labyrinth_cgmath::FloatVec2;
use labyrinth_cgmath::FloatVec3;
use labyrinth_cgmath::prelude::*;

pub enum IndiceType {
    None(NoIndices),
    Buffer(IndexBuffer<u32>),
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3]
}

implement_vertex!(Vertex, position, tex_coords, normal, tangent, bitangent);

pub struct Mesh {
    pub name: String,
    pub material: Shared<Material>,
    pub buffer: VertexBuffer<Vertex>,
    pub indices: IndiceType,
}

pub struct Model {
    pub name: String,
    pub meshes: Vec<String>,
}

trait ToCoords {
    fn to_coords(&self) -> [f32; 3];
}

trait ToTCoords {
    fn to_coords(&self) -> [f32; 2];
}

impl ToCoords for labyrinth_obj::obj::Vertex {
    fn to_coords(&self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }
}

impl ToTCoords for labyrinth_obj::obj::TVertex {
    fn to_coords(&self) -> [f32; 2] {
        [self.u as f32, self.v as f32]
    }
}

impl Model {
    pub fn load<R, F>(mut file: R, facade: &F, context: SharedContext) -> Vec<Model>
    where
        R: std::io::Read,
        F: Facade,
    {
        let mut context = context.borrow_mut();
        let mut model = String::new();
        file.read_to_string(&mut model).unwrap();
        let set = labyrinth_obj::obj::parse(model).unwrap();

        let mut models = Vec::new();

        for obj in set.objects.iter() {
            let mut meshes = Vec::new();
            for (i, mesh) in obj.geometry.iter().enumerate() {
                let mut vertices = Vec::new();
                for shape in mesh.shapes.iter() {
                    if let Triangle(a, b, c) = shape.primitive {
                        let v0 = labyrinth_cgmath::FloatVec3::from(obj.vertices[a.0].to_coords());
                        let v1 = labyrinth_cgmath::FloatVec3::from(obj.vertices[b.0].to_coords());
                        let v2 = labyrinth_cgmath::FloatVec3::from(obj.vertices[c.0].to_coords());

                        let uv0 = labyrinth_cgmath::FloatVec2::from(obj.tex_vertices[a.1.unwrap()].to_coords());
                        let uv1 = labyrinth_cgmath::FloatVec2::from(obj.tex_vertices[b.1.unwrap()].to_coords());
                        let uv2 = labyrinth_cgmath::FloatVec2::from(obj.tex_vertices[c.1.unwrap()].to_coords());

                        let dp1 = v1 - v0;
                        let dp2 = v2 - v0;

                        let duv1 = uv1 - uv0;
                        let duv2 = uv2 - uv0;

                        let r = 1f32 / (duv1.x * duv2.y - duv1.y * duv2.x);
                        let tangent = ((dp1 * duv2.y - dp2 * duv1.y)*r).normalize();
                        let bitangent = ((dp2 * duv1.x - dp1 * duv2.x)*r).normalize();
                        let normal = (v1-v0).cross(v2-v0).normalize();

                        for (vindex, tindex, nindex) in [a, b, c].iter() {
                            vertices.push(Vertex {
                                position: obj.vertices[*vindex].to_coords(),
                                tex_coords: if let Some(index) = tindex {
                                    obj.tex_vertices[*index].to_coords()
                                } else {
                                    [0.0, 0.0]
                                },
                                normal: normal.into(),
                                tangent: tangent.into(),
                                bitangent: bitangent.into()
                            })
                        }
                    }
                }
                let mname = format!("{}{}", obj.name.clone(), i);
                let mesh = Mesh {
                    name: mname.clone(),
                    material: context.get_material(&mesh.material_name.clone().unwrap()).unwrap(),
                    buffer: VertexBuffer::new(facade, &vertices).unwrap(),
                    indices: IndiceType::None(NoIndices(PrimitiveType::TrianglesList)),
                };
                meshes.push(mesh.name.clone());
                context.meshes.insert(mesh.name.clone(), Rc::new(RefCell::new(mesh)));
            }
            models.push(Model {
                name: obj.name.clone(),
                meshes,
            });
        }
        models
    }
}
