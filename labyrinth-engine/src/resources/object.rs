use generational_arena::Index;

use crate::game::context::LabyrinthContext;

use glium::backend::Facade;

use crate::game::rendering::MaterialMap;
use crate::game::rendering::RenderCommand;
use crate::resources::material::EffectBuffer;

use labyrinth_assets::assets::Object;

use crate::resources::{model::MeshBuffer, model::ModelBuffer, shader::ProgramBuffer};

#[derive(Clone)]
pub struct ObjectBuffer {
    pub name: String,
    pub model: Index,
    pub program: Index,
}

impl ObjectBuffer {
    pub fn load<F>(object: &Object, _facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = ObjectBuffer {
            name: object.name.clone(),
            model: ModelBuffer::find(context, &object.model).unwrap(),
            program: ProgramBuffer::find(context, &object.program).unwrap(),
        };
        context.objects.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&ObjectBuffer> {
        context.objects.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.objects.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }

    pub fn render_command(&self, context: &LabyrinthContext) -> Vec<RenderCommand> {
        {
            let model = ModelBuffer::get(context, self.model).unwrap();
            let mut buffer = Vec::new();
            for mesh_idx in model.meshes.iter() {
                let mesh = MeshBuffer::get(context, *mesh_idx).unwrap();
                let effect = EffectBuffer::get(context, mesh.material).unwrap();

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
