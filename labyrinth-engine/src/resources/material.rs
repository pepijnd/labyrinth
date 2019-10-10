use crate::game::context::LabyrinthContext;
use glium::backend::Facade;

use labyrinth_assets::assets::{Effect, Material};

use labyrinth_cgmath::FloatVec3;

use generational_arena::Index;

pub struct MaterialBuffer {
    pub name: String,
    pub effect: Index,
    //pub texture: Shared<Texture>,
}

#[derive(Clone)]
pub struct EffectBuffer {
    pub name: String,
    pub emission: FloatVec3,
    pub ambient: FloatVec3,
    pub diffuse: FloatVec3,
    pub specular: FloatVec3,
    pub shininess: f32,
    pub refraction: f32,
    pub alpha: f32,
}

#[derive(Copy, Clone)]
pub struct MatUnfiform {
    pub effect: EffectUniform,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EffectUniform {
    pub emission: FloatVec3,
    _pad1: [f32; 1],
    pub ambient: FloatVec3,
    _pad2: [f32; 1],
    pub diffuse: FloatVec3,
    _pad3: [f32; 1],
    pub specular: FloatVec3,
    pub shininess: f32,
    pub refraction: f32,
    pub alpha: f32,
}

implement_uniform_block!(
    EffectUniform,
    emission,
    ambient,
    diffuse,
    specular,
    shininess,
    refraction,
    alpha
);

implement_uniform_block!(MatUnfiform, effect);

impl MaterialBuffer {
    pub fn load<F>(material: &Material, _facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = MaterialBuffer {
            name: material.name.clone(),
            effect: EffectBuffer::find(context, &material.effect).unwrap(),
        };
        context.materials.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&MaterialBuffer> {
        context.materials.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.materials.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }

    pub fn to_uniform(&self, context: &LabyrinthContext) -> MatUnfiform {
        MatUnfiform {
            effect: EffectBuffer::get(context, self.effect)
                .unwrap()
                .to_uniform(),
        }
    }
}

impl EffectBuffer {
    pub fn load<F>(effect: &Effect, _facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = EffectBuffer {
            name: effect.name.clone(),
            emission: effect.emission.into(),
            ambient: effect.ambient.into(),
            diffuse: effect.diffuse.into(),
            specular: effect.specular.into(),
            shininess: effect.shininess,
            refraction: effect.refraction,
            alpha: effect.alpha,
        };
        context.effects.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&EffectBuffer> {
        context.effects.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.effects.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }

    pub fn to_uniform(&self) -> EffectUniform {
        EffectUniform {
            emission: self.emission,
            _pad1: [0f32; 1],
            ambient: self.ambient,
            _pad2: [0f32; 1],
            diffuse: self.diffuse,
            _pad3: [0f32; 1],
            specular: self.specular,
            shininess: self.shininess,
            refraction: self.refraction,
            alpha: self.alpha,
        }
    }
}
