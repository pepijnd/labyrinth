use glium::backend::Facade;
use glium::Surface;

pub mod context;
mod counter;
mod instance;
pub mod object;
pub mod rendering;
pub mod camera;
pub mod math;

use counter::Counter;
use instance::Entity;

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
