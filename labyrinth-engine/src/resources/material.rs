pub use labyrinth_obj::mtl::Illumination;
use labyrinth_obj::mtl;

use crate::resources::texture::Texture;
use crate::game::context::Shared;

use labyrinth_cgmath::FloatVec3;

pub struct Material {
    pub name: String,
    pub basematerial: Shared<BaseMaterial>,
    pub texture: Shared<Texture>
}

impl Material {
    pub fn new(name: String, basematerial: Shared<BaseMaterial>, texture: Shared<Texture>) -> Material {
        Material {
            name,
            basematerial,
            texture
        }
    }

    pub fn to_uniform(&self) -> MatUnfiform {
        MatUnfiform {
            base: self.basematerial.borrow().to_uniform()
        }
    }
}

#[derive(Copy, Clone)]
pub struct MatUnfiform {
    pub base: BaseMatUniform
}

#[derive(Clone)]
pub struct BaseMaterial {
    pub name: String,
    pub specular_coefficient: f32,
    pub color_ambient: FloatVec3,
    pub color_diffuse: FloatVec3,
    pub color_specular: FloatVec3,
    pub color_emissive: FloatVec3,
    pub optical_density: f32,
    pub alpha: f32,
    pub illumination: Illumination
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseMatUniform {
    pub specular_coefficient: f32,
    _pad1: [f32; 3],
    pub color_ambient: FloatVec3,
    _pad2: [f32; 1],
    pub color_diffuse: FloatVec3,
    _pad3: [f32; 1],
    pub color_specular: FloatVec3,
    _pad4: [f32; 1],
    pub color_emissive: FloatVec3,
    pub optical_density: f32,
    pub alpha: f32
}

implement_uniform_block!(BaseMatUniform, 
specular_coefficient, 
color_ambient, 
color_diffuse, 
color_specular, 
color_emissive, 
optical_density,
alpha);

implement_uniform_block!(MatUnfiform, base);

impl BaseMaterial {
    pub fn load<R>(mut file: R) -> Vec<BaseMaterial>
    where
        R: std::io::Read
    {
        let mut material = String::new();
        file.read_to_string(&mut material).unwrap();
        let set = labyrinth_obj::mtl::parse(material).unwrap();

        let mut materials = Vec::new();
        for material in set.materials.iter() {
            materials.push(
                BaseMaterial {
                    name: material.name.clone(),
                    specular_coefficient: material.specular_coefficient,
                    color_ambient: material.color_ambient,
                    color_diffuse: material.color_diffuse,
                    color_specular: material.color_specular,
                    color_emissive: material.color_emissive.unwrap_or(FloatVec3::new(0.0, 0.0, 0.0)),
                    optical_density:material.optical_density.unwrap_or(0.0),
                    alpha: material.alpha,
                    illumination: material.illumination
                }
            )
        }
        materials
    }

    pub fn to_uniform(&self) -> BaseMatUniform {
        BaseMatUniform {
            specular_coefficient: self.specular_coefficient,
            _pad1: [0f32; 3],
            color_ambient: self.color_ambient,
            _pad2: [0f32; 1],
            color_diffuse: self.color_diffuse,
            _pad3: [0f32; 1],
            color_specular: self.color_specular,
            _pad4: [0f32; 1],
            color_emissive: self.color_emissive,
            optical_density: self.optical_density,
            alpha: self.alpha
        }
    }
}