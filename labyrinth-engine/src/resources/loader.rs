use std::collections::HashMap;

use glium::backend::Facade;

use labyrinth_assets::assets::Assets;

use crate::game::context::SharedContext;
use crate::game::context::LabyrinthContext;
use crate::resources::{
    Loadable,
    material::EffectBuffer, 
    material::MaterialBuffer, 
    model::ModelBuffer, 
    object::ObjectBuffer,
    shader::ProgramBuffer,
    animation::SkeletonBuffer,
    animation::AnimationBuffer
};

pub struct ResourceLoader {
    context: SharedContext,
    assets: HashMap<String, Assets>,
}

impl ResourceLoader {
    pub fn new(context: SharedContext) -> ResourceLoader {
        ResourceLoader {
            context,
            assets: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, assets: Assets) {
        self.assets.insert(name, assets);
    }

    pub fn load_asset<L, F>(assets: &[L::Source], facade: &F, context: &mut LabyrinthContext)
    where
        L: Loadable,
        F: Facade
    {
        for asset in assets.iter() {
            if let Err(e) = L::load(asset, facade, context) {
                warn!("{}", e);
            }
        }
    }

    pub fn load_assets<F>(&self, facade: &F)
    where
        F: Facade,
    {
        let mut context = self.context.borrow_mut();

        for (_name, assets) in self.assets.iter() {
            Self::load_asset::<ProgramBuffer, F>(&assets.programs, facade, &mut context);
            Self::load_asset::<EffectBuffer, F>(&assets.effects, facade, &mut context);
            Self::load_asset::<MaterialBuffer, F>(&assets.materials, facade, &mut context);
            Self::load_asset::<ModelBuffer, F>(&assets.models, facade, &mut context);
            Self::load_asset::<ObjectBuffer, F>(&assets.objects, facade, &mut context);
            Self::load_asset::<SkeletonBuffer, F>(&assets.skeletons, facade, &mut context);
            Self::load_asset::<AnimationBuffer, F>(&assets.animations, facade, &mut context);
        }
    }
}
