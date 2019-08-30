use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

use image::RgbaImage;

use glium::backend::Facade;
use glium::Texture2d;

use crate::resources::model::Model;
use crate::resources::material::BaseMaterial;
use crate::resources::texture::Texture;
use crate::resources::texture::BaseTexture;
use crate::resources::material::Material;
use crate::game::object::Object;

pub type Shared<T> = Rc<RefCell<T>>;
pub type SharedContext = Shared<LabyrinthContext>;

pub struct LabyrinthContext {
    pub basematerials: HashMap<String, Shared<BaseMaterial>>,
    pub basetextures: HashMap<String, Shared<Texture2d>>,
    pub textures: HashMap<String, Shared<Texture>>,
    pub materials: HashMap<String, Shared<Material>>,
    pub models: HashMap<String, Shared<Model>>,
    pub objects: HashMap<String, Shared<Object>>,
    pub t: f32
}

impl LabyrinthContext {
    pub fn create() -> SharedContext {
        Rc::new(RefCell::new(LabyrinthContext {
            basematerials: HashMap::new(),
            basetextures: HashMap::new(),
            textures: HashMap::new(),
            materials: HashMap::new(),
            models: HashMap::new(),
            objects: HashMap::new(),
            t: 0.0
        }))
    }

    pub fn get_texture(&self, key: &String) -> Option<Shared<Texture>> {
        if let Some(tex) = self.textures.get(key) {
            Some(tex.clone())
        } else { None }
    }

    pub fn get_model(&self, key: &String) -> Option<Shared<Model>> {
        if let Some(model) = self.models.get(key) {
            Some(model.clone())
        } else { None }
    }

    pub fn get_basematerial(&self, key: &String) -> Option<Shared<BaseMaterial>> {
        if let Some(basematerial) = self.basematerials.get(key) {
            Some(basematerial.clone())
        } else { None }
    }

    pub fn get_basetexture(&self, key: &String) -> Option<Shared<BaseTexture>> {
        if let Some(basetexture) = self.basetextures.get(key) {
            Some(basetexture.clone())
        } else { None }
    }

    pub fn get_material(&self, key: &String) -> Option<Shared<Material>> {
        if let Some(material) = self.materials.get(key) {
            Some(material.clone())
        } else { None }
    }

    pub fn get_object(&self, key: &String) -> Option<Shared<Object>> {
        if let Some(object) = self.objects.get(key) {
            Some(object.clone())
        } else { None }
    }
}
