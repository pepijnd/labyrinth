use glium::backend::Facade;
use glium::Surface;

pub mod context;
mod counter;
mod instance;
pub mod object;
pub mod rendering;
pub mod camera;
pub mod entity;

use counter::Counter;
use entity::Entity;

#[derive(Clone)]
pub struct Game {
    counter: Counter,
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Game {
        let mut entities = Vec::new();
        let mut entity = Entity::new("Portal1".to_owned(), "ObjPortal".to_owned());
        entity.scale = 0.25;
        entity.position = labyrinth_cgmath::FloatVec3::new(0.0, 0.0, -4.0);
        entities.push(entity);
        let mut entity = Entity::new("Portal2".to_owned(), "ObjPortal".to_owned());
        entity.scale = 0.25;
        entity.position = labyrinth_cgmath::FloatVec3::new(0.0, 0.0, 4.0);
        entities.push(entity);
        let mut entity = Entity::new("Floor".to_owned(), "ObjFloor".to_owned());
        entity.scale = 5.0;
        entities.push(entity);

        Game {
            counter: Counter::new(),
            entities: entities,
        }
    }

    pub fn update(&mut self) {
        self.counter.count(|| {});
    }
}
