use labyrinth_cgmath::FloatPoint3;
use labyrinth_cgmath::FloatVec3;
use labyrinth_cgmath::FloatMat4;

pub struct Camera {
    position: FloatPoint3,
    look_at: FloatPoint3,
    up: FloatVec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: FloatPoint3::new(-6.0, 4.0, -4.0),
            look_at: FloatPoint3::new(0.0, 0.0, 0.0),
            up: FloatVec3::new(0.0, 1.0, 0.0)
        }
    }

    pub fn get_position(&self) -> FloatPoint3 {
        self.position
    }

    pub fn get_position_mut(&mut self) -> &mut FloatPoint3 {
        &mut self.position
    }

    pub fn get_look_at(&self) -> FloatPoint3 {
        self.look_at
    }

    pub fn get_look_at_mut(&mut self) -> &mut FloatPoint3 {
        &mut self.look_at
    }

    pub fn get_up(&self) -> FloatVec3 {
        self.up
    }

    pub fn look_at(&self) -> FloatMat4 {
        FloatMat4::look_at(self.position, self.look_at, self.up)
    }
}