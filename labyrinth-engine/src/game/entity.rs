use crate::game::context::LabyrinthContext;
use crate::game::rendering::RenderBuffer;
use crate::labyrinth_cgmath::Zero;
use labyrinth_cgmath::{FloatMat4, FloatVec3, Rad};

use crate::resources::object::ObjectBuffer;
use crate::resources::Resource;
use generational_arena::Index;

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub object: Index,
    pub position: FloatVec3,
    pub scale: f32,
    pub rotation: Rad<f32>,
}

impl Entity {
    pub fn new(name: &str, object: Index) -> Entity {
        Entity {
            name: name.to_owned(),
            object,
            position: FloatVec3::new(0.0, 0.0, 0.0),
            scale: labyrinth_cgmath::num_traits::one(),
            rotation: Rad::zero(),
        }
    }

    pub fn render_queue(&self, context: &LabyrinthContext, buffer: &mut RenderBuffer) -> crate::LabyrinthResult<()> {
        let object = ObjectBuffer::get(context, self.object)?;
        let buffers = object.render_command(context)?;
        for mut command in buffers {
            command.matrix = FloatMat4::from_translation(self.position)
                * FloatMat4::from_angle_y(self.rotation)
                * FloatMat4::from_scale(self.scale);
            buffer.push(command);
        }

        Ok(())
    }
}
