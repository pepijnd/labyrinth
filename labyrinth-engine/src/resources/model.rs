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

use crate::LabyrinthResult;
use crate::resources::ResourceError;

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

    fn load<F>(mesh: &Mesh, facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade,
    {
        let buffer = MeshBuffer {
            name: mesh.name.clone(),
            material: MaterialBuffer::find(context, &mesh.material).map_err(|e| 
                ResourceError::Loading(e, mesh.name.clone(), Self::get_type()))?,
            vertices: VertexBuffer::new(facade, &mesh.vertices)
            .map_err(|e| ResourceError::Render(e.into(), mesh.name.clone(), Self::get_type()))?
        };
        
        Ok(context.resources.insert(Box::new(buffer)))
    }
}

impl Loadable for ModelBuffer {
    type Source = Model;

    fn load<F>(model: &Model, facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade,
    {
        let buffer = ModelBuffer {
            name: model.name.clone(),
            meshes: model
                .meshes
                .iter()
                .map(|x| MeshBuffer::load(x, facade, context))
                .fold(Ok(Vec::new()), |a: LabyrinthResult<Vec<Index>>, b: LabyrinthResult<Index>| 
                    a.and_then(|mut x| {x.push(b?); Ok(x)})
                )?
        };
        
        Ok(context.resources.insert(Box::new(buffer)))
    }
}
