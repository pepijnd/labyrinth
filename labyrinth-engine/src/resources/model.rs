use crate::game::context::LabyrinthContext;

use glium::backend::Facade;
use glium::VertexBuffer;

use crate::resources::material::MaterialBuffer;

use generational_arena::Index;

use labyrinth_assets::assets::{Mesh, Model, Vertex};

pub struct MeshBuffer {
    pub name: String,
    pub material: Index,
    pub vertices: VertexBuffer<Vertex>,
}

pub struct ModelBuffer {
    pub name: String,
    pub meshes: Vec<Index>,
}

impl MeshBuffer {
    pub fn load<F>(mesh: &Mesh, facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = MeshBuffer {
            name: mesh.name.clone(),
            material: MaterialBuffer::find(context, &mesh.material).unwrap(),
            vertices: VertexBuffer::new(facade, &mesh.vertices).unwrap(),
        };
        context.meshes.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&MeshBuffer> {
        context.meshes.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.meshes.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }
}

impl ModelBuffer {
    pub fn load<F>(model: &Model, facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = ModelBuffer {
            name: model.name.clone(),
            meshes: model
                .meshes
                .iter()
                .map(|x| MeshBuffer::load(x, facade, context))
                .collect(),
        };
        context.models.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&ModelBuffer> {
        context.models.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.models.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }
}
