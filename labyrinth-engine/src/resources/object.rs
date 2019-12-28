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
    Findable, Loadable,
    model::MeshBuffer, 
    model::ModelBuffer, 
    shader::ProgramBuffer
};

#[derive(Clone, Debug)]
pub struct ObjectBuffer {
    pub name: String,
    pub model: Index,
    pub program: Index,
}

impl_resource!(ObjectBuffer, name);

impl Loadable for ObjectBuffer {
    type Source = Object;

    fn load<F>(object: &Object, _facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = ObjectBuffer {
            name: object.name.clone(),
            model: ModelBuffer::find(context, &object.model).unwrap(),
            program: ProgramBuffer::find(context, &object.program).unwrap(),
        };
        context.resources.insert(Box::new(buffer))
    }
}

impl ObjectBuffer {
    pub fn render_command(&self, context: &LabyrinthContext) -> Vec<RenderCommand> {
        {
            let model = ModelBuffer::get(context, self.model).unwrap();
            let mut buffer = Vec::new();
            for mesh_idx in model.meshes.iter() {
                let mesh = MeshBuffer::get(context, *mesh_idx).unwrap();
                let mat = MaterialBuffer::get(context, mesh.material).unwrap();
                let effect = EffectBuffer::get(context, mat.effect).unwrap();

                buffer.push(RenderCommand::new(
                    MaterialMap::new(&effect),
                    *mesh_idx,
                    // material.texture.borrow().name.clone(),
                    self.program,
                ));
            }
            buffer
        }
    }
}
