use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub name: String,
    pub model: String,
    pub program: String,
}
