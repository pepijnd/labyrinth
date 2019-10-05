use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub emission: (f32, f32, f32),
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
    pub shininess: f32,
    pub refraction: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Material {
    pub name: String,
    pub effect: String,
}
