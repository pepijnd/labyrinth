use crate::game::context::SharedContext;
use crate::game::rendering::RenderBuffer;
use crate::labyrinth_cgmath::Zero;
use labyrinth_cgmath::{
    FloatMat4,
    FloatVec3,
    Rad,
};

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    pub object: String,
    pub position: FloatVec3,
    pub scale: f32,
    pub rotation: Rad<f32>
}

impl Entity {
    pub fn new(name: String, object: String) -> Entity {
        Entity {
            name,
            object,
            position: FloatVec3::new(0.0, 0.0, 0.0),
            scale: labyrinth_cgmath::num_traits::one(),
            rotation: Rad::zero(),
        }
    }

    pub fn render_queue(&self, context: SharedContext, buffer: &mut RenderBuffer) {
        let shared = context.clone();
        let context = context.borrow();
        let object = context.get_object(&self.object).unwrap();
        let object = object.borrow();
        let buffers = object.render_command(shared);
        for mut command in buffers {
            command.matrix = FloatMat4::from_scale(self.scale) * 
                             FloatMat4::from_angle_y(self.rotation) * 
                             FloatMat4::from_translation(self.position);
            buffer.push(command);
        }
    }
}