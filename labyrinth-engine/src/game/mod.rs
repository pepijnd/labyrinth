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
        let mut entity = Entity::new("Portal".to_owned(), "ObjPortal".to_owned());
        entity.scale = 0.25;
        entity.position = labyrinth_cgmath::FloatVec3::new(0.25, 0.0, 2.0);
        entities.push(entity);
        let mut entity = Entity::new("Torch".to_owned(), "ObjTorch".to_owned());
        entity.scale = 1.0;
        entity.position = labyrinth_cgmath::FloatVec3::new(2.0, 1.5, 2.7);
        entity.rotation = labyrinth_cgmath::Angle::turn_div_3();
        entities.push(entity);
        for x in 0..10 {
            for y in 0..10 {
                let mut entity = Entity::new(format!("Block{}{}", x, y).to_owned(), "ObjBlock".to_owned());
                entity.scale = 0.5;
                entity.position = labyrinth_cgmath::FloatVec3::new((x-5) as f32, -0.5, (y-5) as f32);
                entities.push(entity);
            }
        }
        let mut entity = Entity::new("Block".to_owned(), "ObjBlock".to_owned());
        entity.scale = 0.3;
        entity.position = labyrinth_cgmath::FloatVec3::new(0.0, 0.3, 0.0);
        entities.push(entity);
        let mut entity = Entity::new("Block2".to_owned(), "ObjBlock".to_owned());
        entity.scale = 0.7;
        entity.position = labyrinth_cgmath::FloatVec3::new(-1.5, 0.7, -3.0);
        //entities.push(entity);

        Game {
            counter: Counter::new(),
            entities: entities,
        }
    }

    pub fn update(&mut self) {
        self.counter.count(|| {});
    }
}
