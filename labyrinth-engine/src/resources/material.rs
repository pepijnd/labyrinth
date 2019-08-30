pub use labyrinth_obj::mtl::Illumination;
use labyrinth_obj::mtl;

use crate::resources::texture::Texture;
use crate::game::context::Shared;

trait ToColor {
    fn to_color(&self) -> [f32; 3];
}

impl ToColor for mtl::Color {
    fn to_color(&self) -> [f32; 3] {
        [self.r as f32, self.g as f32, self.b as f32]
    }
}

pub struct Material {
    pub name: String,
    pub basematerial: Shared<BaseMaterial>,
    pub texture: Option<Shared<Texture>>
}

impl Material {
    pub fn new(name: String, basematerial: Shared<BaseMaterial>, texture: Option<Shared<Texture>>) -> Material {
        Material {
            name,
            basematerial,
            texture
        }
    }
}

pub struct BaseMaterial {
    pub name: String,
    pub specular_coefficient: f64,
    pub color_ambient: [f32; 3],
    pub color_diffuse: [f32; 3],
    pub color_specular: [f32; 3],
    pub color_emissive: Option<[f32; 3]>,
    pub optical_density: Option<f32>,
    pub alpha: f32,
    pub illumination: Illumination,
    pub uv_map: Option<String>
}

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
                    color_ambient: material.color_ambient.to_color(),
                    color_diffuse: material.color_diffuse.to_color(),
                    color_specular: material.color_specular.to_color(),
                    color_emissive: if let Some(color) = material.color_emissive {
                                        Some(color.to_color())
                                    } else { None },
                    optical_density: if let Some(density) = material.optical_density  {
                                        Some(density as f32)
                                    } else { None },
                    alpha: material.alpha as f32,
                    illumination: material.illumination,
                    uv_map: material.uv_map.clone()
                }
            )
        }
        materials
    }
}