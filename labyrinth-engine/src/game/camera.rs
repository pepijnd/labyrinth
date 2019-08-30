use glm::ext::look_at;
use crate::game::math::Uniform;

pub struct Camera {
    position: glm::Vec3,
    look_at: glm::Vec3,
    up: glm::Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glm::vec3(-6.0, 4.0, -4.0),
            look_at: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 1.0, 0.0)
        }
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn get_position_mut(&mut self) -> &mut glm::Vec3 {
        &mut self.position
    }

    pub fn get_look_at(&self) -> glm::Vec3 {
        self.look_at
    }

    pub fn get_up(&self) -> glm::Vec3 {
        self.up
    }

    pub fn make_view(&self) -> [[f32; 4]; 4] {
        glm::ext::look_at(self.position, self.look_at, self.up).as_uniform()
    }
}