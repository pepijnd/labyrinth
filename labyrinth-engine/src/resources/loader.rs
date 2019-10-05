use glium::backend::Facade;
use std::collections::HashMap;

use crate::game::context::SharedContext;
use crate::resources::{
    material::EffectBuffer, material::MaterialBuffer, model::ModelBuffer, object::ObjectBuffer,
    shader::ProgramBuffer,
};

use labyrinth_assets::assets::Assets;

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

    pub fn load_assets<F>(&self, facade: &F)
    where
        F: Facade,
    {
        let mut context = self.context.borrow_mut();

        for (_name, assets) in self.assets.iter() {
            for program in assets.programs.iter() {
                ProgramBuffer::load(program, facade, &mut context);
            }

            for effect in assets.effects.iter() {
                EffectBuffer::load(effect, facade, &mut context);
            }

            for material in assets.materials.iter() {
                MaterialBuffer::load(material, facade, &mut context);
            }

            for model in assets.models.iter() {
                ModelBuffer::load(model, facade, &mut context);
            }

            for object in assets.objects.iter() {
                ObjectBuffer::load(object, facade, &mut context);
            }
        }
    }
}
