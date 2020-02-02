use glium::backend::Facade;
use crate::resources::ResourceIndex;
use crate::resources::ResourceBase;

use labyrinth_assets::assets::{
    Skeleton,
    Joint,
    Animation,
    AnimationTarget
};

use labyrinth_cgmath::FloatMat4;

use crate::game::context::LabyrinthContext;
use crate::impl_resource;
use crate::resources::Loadable;

#[derive(Debug)]
pub struct SkeletonBuffer {
    pub name: String,
    pub joints: Vec<Joint>,
    pub bind_poses: Vec<FloatMat4>,
}

impl_resource!(SkeletonBuffer, name);

impl Loadable for SkeletonBuffer {
    type Source = Skeleton;

    fn load<F, T>(skeleton: &Skeleton, _facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<ResourceIndex<SkeletonBuffer>>
    where
        F: Facade,
        T: ResourceBase
    {
        let buffer = SkeletonBuffer {
            name: skeleton.name.clone(),
            joints: skeleton.joints.clone(),
            bind_poses: skeleton.bind_poses.clone()
        };
        
        Ok(context.resources.insert(buffer))
    }
}

#[derive(Debug)]
pub struct AnimationBuffer {
    pub name: String,
    pub targets: Vec<AnimationTarget>,
}

impl_resource!(AnimationBuffer, name);

impl Loadable for AnimationBuffer {
    type Source = Animation;

    fn load<F>(animation: &Animation, _facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade,
    {
        let buffer = AnimationBuffer {
            name: animation.name.clone(),
            targets: animation.targets.clone()
        };
        
        Ok(context.resources.insert(Box::new(buffer)))
    }
}