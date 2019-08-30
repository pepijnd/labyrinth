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

pub enum IndiceType {
    None(NoIndices),
    Buffer(IndexBuffer<u32>),
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, tex_coords, normal);

pub struct Mesh {
    pub material: Option<Shared<Material>>,
    pub buffer: VertexBuffer<Vertex>,
    pub indices: IndiceType,
}

pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
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
            for mesh in obj.geometry.iter() {
                let mut vertices = Vec::new();
                for shape in mesh.shapes.iter() {
                    if let Triangle(a, b, c) = shape.primitive {
                        for (vindex, tindex, nindex) in [a, b, c].iter() {
                            vertices.push(Vertex {
                                position: obj.vertices[*vindex].to_coords(),
                                tex_coords: if let Some(index) = tindex {
                                    obj.tex_vertices[*index].to_coords()
                                } else {
                                    [0.0, 0.0]
                                },
                                normal: if let Some(index) = nindex {
                                    obj.normals[*index].to_coords()
                                } else {
                                    [0.0, 0.0, 0.0]
                                },
                            })
                        }
                    }
                }
                meshes.push(Mesh {
                    material: if let Some(name) = &mesh.material_name {
                                  Some(context.get_material(name).unwrap()) } 
                              else { None },
                    buffer: VertexBuffer::new(facade, &vertices).unwrap(),
                    indices: IndiceType::None(NoIndices(PrimitiveType::TrianglesList)),
                });
            }
            models.push(Model {
                name: obj.name.clone(),
                meshes,
            });
        }
        models
    }
}
