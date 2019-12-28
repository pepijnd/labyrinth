use std::cell::RefCell;
use std::rc::Rc;

use generational_arena::Arena;

use crate::resources::{
    ResourceBase,
};

pub type Shared<T> = Rc<RefCell<T>>;
pub type SharedContext = Shared<LabyrinthContext>;

pub struct LabyrinthContext {
    pub resources: Arena<Box<dyn ResourceBase>>,
    pub t: f32,
}

impl LabyrinthContext {
    pub fn create() -> SharedContext {
        let context = LabyrinthContext {
            resources: Arena::new(),
            t: 0.0,
        };

        Rc::new(RefCell::new(context))
    }
}
