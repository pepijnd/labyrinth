use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shader {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub name: String,
    pub vertex: Shader,
    pub fragment: Shader,
}
