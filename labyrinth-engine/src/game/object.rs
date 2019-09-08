use crate::resources::model::Model;
use crate::resources::shader::Program;
use crate::game::rendering::RenderCommand;
use crate::game::rendering::MaterialMap;
use crate::game::context::SharedContext;
use crate::game::context::Shared;

#[derive(Clone)]
pub struct Object {
    pub name: String,
    pub model: Shared<Model>,
    pub program: Shared<Program>
}

impl Object {
    pub fn new(name: String, model: Shared<Model>, program: Shared<Program>) -> Object {
        Object {
            name,
            model,
            program
        }
    }

    pub fn render_command(&self, context: SharedContext) -> Vec<RenderCommand> {
        {
            let context = context.borrow();
            let mut buffer = Vec::new();
            for mesh in self.model.borrow().meshes.iter() {
                let m = context.get_mesh(mesh).unwrap();
                let m = m.borrow();               
                let material = m.material.borrow();

                buffer.push(RenderCommand::new(
                    MaterialMap::new(&material.basematerial.borrow()),
                    mesh.clone(),
                    material.texture.borrow().name.clone(),
                    self.program.borrow().name.clone()
                ));
            }
            buffer
        }
    }
}
