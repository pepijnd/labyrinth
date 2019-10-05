pub mod camera;
pub mod context;
mod counter;
pub mod entity;
pub mod rendering;

use counter::Counter;
use entity::Entity;

#[derive(Clone)]
pub struct Game {
    counter: Counter,
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            counter: Counter::new(),
            entities: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.counter.count(|| {});
    }
}
