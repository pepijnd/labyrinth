pub trait Uniform {
    fn as_uniform(&self) -> [[f32; 4]; 4];
}

impl Uniform for glm::Mat4 {
    fn as_uniform(&self) -> [[f32; 4]; 4] {
        let mut array = [[0.0; 4]; 4];
        for (i, row) in self.as_array().iter().enumerate() {
            array[i].copy_from_slice(row.as_array());
        }
        array
    }
}
