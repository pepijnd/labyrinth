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
use crate::resources::shader::Shader;
use crate::resources::shader::Program;
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
                            self.load_basematerial_file(file);
                        }
                    }
                }
            } else { panic!() }
            if let Some(textures) = assets.get("textures") {
                if let JsonValue::Array(textures) = textures {
                    for texture in textures.iter() {
                        if let JsonValue::Object(texture) = texture {
                            let name = texture.get("name").unwrap().as_str().unwrap();
                            let basetexture = texture.get("basetexture").unwrap().as_str().unwrap();
                            let normal = texture.get("normal").unwrap().as_str().unwrap();
                            let basetexture = self.context.borrow().get_basetexture(&String::from(basetexture)).unwrap();
                            let normal = self.context.borrow().get_basetexture(&String::from(normal)).unwrap();
                            self.load_texture(Texture {
                                name: String::from(name),
                                basetexture,
                                normal
                            });
                        }
                    }
                }
            } else { panic!() }
            if let Some(materials) = assets.get("materials") {
                if let JsonValue::Array(materials) = materials {
                    for material in materials.iter() {
                        if let JsonValue::Object(material) = material {
                            let name = material.get("name").unwrap().as_str().unwrap();
                            let basematerial = material.get("basematerial").unwrap().as_str().unwrap();
                            let texture = material.get("texture").unwrap();
                            let basematerial = self.context.borrow().get_basematerial(&String::from(basematerial)).unwrap();
                            let texture = self.context.borrow().get_texture(&String::from(texture.as_str().unwrap())).unwrap();
                            self.load_material(Material {
                                name: String::from(name),
                                basematerial,
                                texture
                            });
                        }
                    }
                }
            } else { panic!() }
            if let Some(models) = assets.get("models") {
                if let JsonValue::Array(models) = models {
                    for model in models.iter() {
                        if let JsonValue::Object(model) = model {
                            let name = model.get("name").unwrap().as_str().unwrap();
                            let source = model.get("source").unwrap().as_str().unwrap();
                            let mut path = dir.clone();
                            path.push(source);
                            let file = File::open(path).unwrap();
                            let file = BufReader::new(file);
                            self.load_model_file(facade, String::from(name), file);
                        }
                    }
                }
            } else { panic!() }
            if let Some(shaders) = assets.get("shaders") {
                if let JsonValue::Array(shaders) = shaders {
                    for shader in shaders.iter() {
                        if let JsonValue::Object(shader) = shader {
                            let name = shader.get("name").unwrap().as_str().unwrap();
                            let source = shader.get("source").unwrap().as_str().unwrap();
                            let mut path = dir.clone();
                            path.push(source);
                            let mut file = std::fs::File::open(path).unwrap();
                            let mut source = String::new();
                            file.read_to_string(&mut source).unwrap();
                            self.load_shader(Shader {
                                name: String::from(name),
                                source
                            });
                        }
                    }
                }
            } else { panic!() }
            if let Some(programs) = assets.get("programs") {
                if let JsonValue::Array(programs) = programs {
                    for program in programs.iter() {
                        if let JsonValue::Object(program) = program {
                            let name = program.get("name").unwrap().as_str().unwrap();
                            let vertex = program.get("vertex").unwrap().as_str().unwrap();
                            let fragment = program.get("fragment").unwrap().as_str().unwrap();
                            let geometry = program.get("geometry").as_ref().map(|x| x.as_str().unwrap());
                            let vertex = self.context.borrow().get_shader(&String::from(vertex)).unwrap();
                            let fragment = self.context.borrow().get_shader(&String::from(fragment)).unwrap();
                            let geometry = geometry.as_ref().map(|x| self.context.borrow().get_shader(&String::from(*x)).unwrap());
                            self.load_program(Program::new(String::from(name), facade, self.context.clone(), vertex, fragment, geometry));
                        }
                    }
                }
            } else { panic!() }
            if let Some(objects) = assets.get("objects") {
                if let JsonValue::Array(objects) = objects {
                    for object in objects.iter() {
                        if let JsonValue::Object(object) = object {
                            let name = object.get("name").unwrap().as_str().unwrap();
                            let model = object.get("model").unwrap().as_str().unwrap();
                            let model = self.context.borrow().get_model(&String::from(model)).unwrap();
                            let program = object.get("program").unwrap().as_str().unwrap();
                            let program = self.context.borrow().get_program(&String::from(program)).unwrap();
                            self.load_object(Object {
                                name: String::from(name),
                                model,
                                program
                            });
                        }
                    }
                }
            } else { panic!() }
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

    pub fn load_model_file<F, R>(&mut self, facade: &F, name: String, model: R)
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
        self.load_model(model);
    }

    pub fn load_model(&mut self, model: Model)
    {
        self.context.borrow_mut().models.insert(model.name.clone(), Rc::new(RefCell::new(model)));
    }

    pub fn load_basematerial_file<R>(&mut self, material: R)
    where
        R: std::io::Read
    {
        for material in BaseMaterial::load(material) {
            self.load_basematerial(material);
        }
    }
        
    pub fn load_basematerial(&mut self, material: BaseMaterial)
    {
        self.context.borrow_mut().basematerials.insert(material.name.clone(), Rc::new(RefCell::new(material)));
    }

    pub fn load_material(&mut self, material: Material) {
        self.context.borrow_mut().materials.insert(material.name.clone(), Rc::new(RefCell::new(material)));
    }

    pub fn load_shader(&mut self, shader: Shader) {
        self.context.borrow_mut().shaders.insert(shader.name.clone(), Rc::new(RefCell::new(shader)));
    }

    pub fn load_program(&mut self, program: Program) {
        self.context.borrow_mut().programs.insert(program.name.clone(), Rc::new(RefCell::new(program)));
    }

    pub fn load_object(&mut self, object: Object) {
        self.context.borrow_mut().objects.insert(object.name.clone(), Rc::new(RefCell::new(object)));
    }
}