use generational_arena::Index;
use glium::backend::Facade;

use labyrinth_assets::assets::Object;

use crate::game::context::LabyrinthContext;

use crate::game::rendering::MaterialMap;
use crate::game::rendering::RenderCommand;
use crate::resources::material::EffectBuffer;
use crate::resources::material::MaterialBuffer;
use crate::impl_resource;
use crate::resources::{
    Resource, Loadable,
    model::MeshBuffer, 
    model::ModelBuffer, 
    shader::ProgramBuffer
};

use crate::resources::ResourceError;

#[derive(Clone, Debug)]
pub struct ObjectBuffer {
    pub name: String,
    pub model: Index,
    pub program: Index,
}

impl_resource!(ObjectBuffer, name);

impl Loadable for ObjectBuffer {
    type Source = Object;

    fn load<F>(object: &Object, _facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade,
    {
        let buffer = ObjectBuffer {
            name: object.name.clone(),
            model: ModelBuffer::find(context, &object.model).map_err(|e| 
                ResourceError::Loading(e, object.name.clone(), Self::get_type()))?,
            program: ProgramBuffer::find(context, &object.program).map_err(|e| 
                ResourceError::Loading(e, object.name.clone(), Self::get_type()))?,
        };
        
        Ok(context.resources.insert(Box::new(buffer)))
    }
}

impl ObjectBuffer {
    pub fn render_command(&self, context: &LabyrinthContext) -> crate::LabyrinthResult<Vec<RenderCommand>> {
        {
            let model = ModelBuffer::get(context, self.model)?;
            let mut buffer = Vec::new();
            for mesh_idx in model.meshes.iter() {
                let mesh = MeshBuffer::get(context, *mesh_idx)?;
                let mat = MaterialBuffer::get(context, mesh.material)?;
                let effect = EffectBuffer::get(context, mat.effect)?;

                buffer.push(RenderCommand::new(
                    MaterialMap::new(&effect),
                    *mesh_idx,
                    // material.texture.borrow().name.clone(),
                    self.program,
                ));
            }
            
            Ok(buffer)
        }
    }
}
