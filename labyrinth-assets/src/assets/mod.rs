use serde::Deserialize;
use serde::Serialize;

pub mod animation;
pub mod material;
pub mod model;
pub mod object;
pub mod shaders;

pub use animation::{Animation, AnimationTarget, Joint, Skeleton};
pub use material::{Effect, Material};
pub use model::{Mesh, Model, Vertex, Vertices};
pub use object::Object;
pub use shaders::{Program, Shader};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assets {
    pub models: Vec<Model>,
    pub skeletons: Vec<Skeleton>,
    pub animations: Vec<Animation>,
    pub programs: Vec<Program>,
    pub objects: Vec<Object>,
    pub effects: Vec<Effect>,
    pub materials: Vec<Material>,
}
