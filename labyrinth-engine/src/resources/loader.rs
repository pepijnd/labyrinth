use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


use image::RgbaImage;
use glium::backend::Facade;

use json::JsonValue;

use crate::game::context::SharedContext;
use crate::resources::model::Model;
use crate::resources::material::BaseMaterial;
use crate::resources::material::Material;
use crate::resources::texture::Texture;
use crate::resources::texture::BaseTexture;
use crate::game::object::Object;

pub struct ResourceLoader {
    context: SharedContext
}

impl ResourceLoader {
    pub fn new(context: SharedContext) -> ResourceLoader {
        ResourceLoader {
            context
        }
    }

    pub fn load_file<F, R>(&mut self, facade : &F, mut file: R) 
    where 
        R: std::io::Read,
        F: Facade
    
    {
        let mut dir = std::env::current_dir().unwrap();
        dir.push("assets/");

        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let parsed = json::parse(input.as_str()).unwrap();
        if let JsonValue::Object(assets) = parsed {
            if let Some(basetextures) = assets.get("basetextures") {
                if let JsonValue::Array(basetextures) = basetextures {
                    for basetexture in basetextures.iter() {
                        if let JsonValue::Object(basetexture) = basetexture {
                            let name = basetexture.get("name").unwrap().as_str().unwrap();
                            let source = basetexture.get("source").unwrap().as_str().unwrap();
                            let mut path = dir.clone();
                            path.push(source);
                            let file = File::open(path).unwrap();
                            let file = BufReader::new(file);
                            self.load_basetexture(facade, String::from(name), image::load(file, image::PNG).unwrap().to_rgba());
                        }
                    }
                }
            } else { panic!() }
            if let Some(basematerials) = assets.get("basematerials") {
                if let JsonValue::Array(basematerials) = basematerials {
                    for basematerial in basematerials.iter() {
                        if let JsonValue::Object(basematerial) = basematerial {
                            let source = basematerial.get("source").unwrap().as_str().unwrap();
                            let mut path = dir.clone();
                            path.push(source);
                            let file = File::open(path).unwrap();
                            let file = BufReader::new(file);
                            self.load_basematerial(file);
                        }
                    }
                }
            } else { panic!() }
            if let Some(textures) = assets.get("textures") {
                if let JsonValue::Array(textures) = textures {
                    for texture in textures.iter() {
                        if let JsonValue::Object(texture) = texture {
                            
                        }
                    }
                }
            }
        }
    }

    pub fn load_basetexture<F>(&mut self, facade: &F, key: String, image: RgbaImage)
    where
        F: Facade,
    {
        let dims = image.dimensions();
        let raw =
            glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw().as_slice(), dims);
        let texture = BaseTexture::new(facade, raw).unwrap();
        self.context.borrow_mut().basetextures.insert(key, Rc::new(RefCell::new(texture)));
    }

    pub fn load_texture(&mut self, texture: Texture) {
        self.context.borrow_mut().textures.insert(texture.name.clone(), Rc::new(RefCell::new(texture)));
    }

    pub fn load_model<F, R>(&mut self, facade: &F, name: String, model: R)
    where
        F: Facade,
        R: std::io::Read,
    {
        let mut models = Model::load(model, facade, self.context.clone());
        if models.len() > 1 {
            println!("warning multiple objects in file");
        }
        let mut model = models.pop().unwrap();
        model.name = name.clone();
        self.context.borrow_mut().models.insert(name, Rc::new(RefCell::new(model)));
    }

    pub fn load_basematerial<R>(&mut self, material: R)
    where
        R: std::io::Read
    {
        for material in BaseMaterial::load(material) {
            self.context.borrow_mut().basematerials.insert(material.name.clone(), Rc::new(RefCell::new(material)));
        }
    }

    pub fn load_material(&mut self, material: Material) {
        self.context.borrow_mut().materials.insert(material.name.clone(), Rc::new(RefCell::new(material)));
    }

    pub fn load_object(&mut self, object: Object) {
        self.context.borrow_mut().objects.insert(object.name.clone(), Rc::new(RefCell::new(object)));
    }
}