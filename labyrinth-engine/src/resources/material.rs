use glium::backend::Facade;
use generational_arena::Index;

use labyrinth_assets::assets::{Effect, Material};
use labyrinth_cgmath::FloatVec3;

use crate::game::context::LabyrinthContext;
use crate::resources::{
    Loadable, Findable
};
use crate::impl_resource;

#[derive(Debug)]
pub struct MaterialBuffer {
    pub name: String,
    pub effect: Index,
    //pub texture: Shared<Texture>,
}

impl_resource!(MaterialBuffer, name);

#[derive(Clone, Debug)]
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

impl_resource!(EffectBuffer, name);

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

impl Loadable for MaterialBuffer {
    type Source = Material;

    fn load<F>(material: &Material, _facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade,
    {
        let buffer = MaterialBuffer {
            name: material.name.clone(),
            effect: EffectBuffer::find(context, &material.effect).unwrap(),
        };
        context.resources.insert(Box::new(buffer))
    }
}



impl MaterialBuffer {
    pub fn to_uniform(&self, context: &LabyrinthContext) -> MatUnfiform {
        MatUnfiform {
            effect: EffectBuffer::get(context, self.effect)
                .unwrap()
                .to_uniform(),
        }
    }
}



impl Loadable for EffectBuffer {
    type Source = Effect;

    fn load<F>(effect: &Effect, _facade: &F, context: &mut LabyrinthContext) -> Index
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
        context.resources.insert(Box::new(buffer))
    }
}


impl EffectBuffer {
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
