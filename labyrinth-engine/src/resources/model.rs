use glium::backend::Facade;
use glium::VertexBuffer;

use generational_arena::Index;

use labyrinth_assets::assets::{Mesh, Model, Vertex};

use crate::game::context::LabyrinthContext;
use crate::resources::material::MaterialBuffer;
use crate::impl_resource;
use crate::resources::{
    Findable, Loadable
};

#[derive(Debug)]
pub struct MeshBuffer {
    pub name: String,
    pub material: Index,
    pub vertices: VertexBuffer<Vertex>,
}

impl_resource!(MeshBuffer, name);

#[derive(Debug)]
pub struct ModelBuffer {
    pub name: String,
    pub meshes: Vec<Index>,
}

impl_resource!(ModelBuffer, name);

impl Loadable for MeshBuffer {
    type Source = Mesh;

    fn load<F>(mesh: &Mesh, facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = MeshBuffer {
            name: mesh.name.clone(),
            material: MaterialBuffer::find(context, &mesh.material).unwrap(),
            vertices: VertexBuffer::new(facade, &mesh.vertices).unwrap(),
        };
        context.resources.insert(Box::new(buffer))
    }
}

impl Loadable for ModelBuffer {
    type Source = Model;

    fn load<F>(model: &Model, facade: &F, context: &mut LabyrinthContext) -> Index
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
        context.resources.insert(Box::new(buffer))
    }
}
