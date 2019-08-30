use std::rc::Rc;
use std::cell::RefCell;

use crate::attributes::{Position, Rotation};

use crate::game::object::Object;

#[derive(Copy, Clone)]
pub struct Instance {}

#[derive(Clone)]
pub struct Entity {
    objects: Vec<Instance>,

    position: Position,
    Rotation: Rotation,
}

impl Entity {
    fn new() -> Entity {
        Entity {
            objects: Vec::new(),

            position: Default::default(),
            Rotation: Default::default(),
        }
    }
}
