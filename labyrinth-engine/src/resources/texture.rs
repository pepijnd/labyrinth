use glium::Texture2d;

use crate::game::context::Shared;

pub type BaseTexture = Texture2d;

pub struct Texture {
    pub name: String,
    pub basetexture: Shared<Texture2d>,
    pub normal: Option<Shared<BaseTexture>>
}

impl Texture {
    pub fn new(name: String, basetexture: Shared<Texture2d>, normal: Option<Shared<Texture2d>>) -> Texture {
        Texture {
            name,
            basetexture,
            normal,
        }
    }
}