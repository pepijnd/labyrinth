use smallvec::SmallVec;

#[derive(Copy, Clone, Default)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone, Default)]
pub struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position { x, y, z }
    }

    pub fn as_vec(&self) -> SmallVec<[f32; 3]> {
        SmallVec::from_slice(&[self.x, self.y, self.z])
    }
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Rotation {
        Rotation { x, y, z }
    }

    pub fn as_slice(&self) -> SmallVec<[f32; 3]> {
        SmallVec::from_slice(&[self.x, self.y, self.z])
    }
}
