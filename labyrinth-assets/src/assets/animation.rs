use serde::Deserialize;
use serde::Serialize;

use labyrinth_cgmath::FloatMat4;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joint {
    pub name: String,
    pub parent_index: Option<u8>,
    pub inverse_bind_pose: FloatMat4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skeleton {
    pub name: String,
    pub joints: Vec<Joint>,
    pub bind_poses: Vec<FloatMat4>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub targets: Vec<AnimationTarget>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnimationTarget {
    pub target: String,
    pub sample_poses: Vec<FloatMat4>,
}
