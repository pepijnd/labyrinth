use std::cell::RefCell;
use std::rc::Rc;

use generational_arena::Arena;

use crate::resources::{
    material::EffectBuffer, material::MaterialBuffer, model::MeshBuffer, model::ModelBuffer,
    object::ObjectBuffer, shader::ProgramBuffer,
};

pub type Shared<T> = Rc<RefCell<T>>;
pub type SharedContext = Shared<LabyrinthContext>;

pub struct LabyrinthContext {
    pub meshes: Arena<MeshBuffer>,
    pub models: Arena<ModelBuffer>,
    pub programs: Arena<ProgramBuffer>,
    pub objects: Arena<ObjectBuffer>,
    pub effects: Arena<EffectBuffer>,
    pub materials: Arena<MaterialBuffer>,
    pub t: f32,
}

impl LabyrinthContext {
    pub fn create() -> SharedContext {
        let context = LabyrinthContext {
            meshes: Arena::new(),
            models: Arena::new(),
            programs: Arena::new(),
            objects: Arena::new(),
            effects: Arena::new(),
            materials: Arena::new(),
            t: 0.0,
        };

        Rc::new(RefCell::new(context))
    }
}
