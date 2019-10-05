use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramRepr {
    pub name: String,
    pub vertex: String,
    pub fragment: String,
}
