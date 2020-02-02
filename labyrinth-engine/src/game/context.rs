use std::cell::RefCell;
use std::rc::Rc;

use generational_arena::Arena;

use crate::resources::{
    ResourceBase,
};

use crate::resources::ResourceArena;

pub type Shared<T> = Rc<RefCell<T>>;
pub type SharedContext = Shared<LabyrinthContext>;

pub struct LabyrinthContext {
    pub resources: ResourceArena,
    pub t: f32,
}

impl LabyrinthContext {
    pub fn create() -> SharedContext {
        let context = LabyrinthContext {
            resources: ResourceArena::new(),
            t: 0.0,
        };

        Rc::new(RefCell::new(context))
    }
}
