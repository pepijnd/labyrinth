use std::cell::RefCell;
use std::sync::{Arc, RwLock};

use glutin::event_loop::{ControlFlow, EventLoopWindowTarget};

use glutin::event::Event as GlutinEvent;

use glutin::event::{StartCause, WindowEvent};

use glium::Surface;

mod event;
pub use event::Event;

use crate::game::context::SharedContext;
use crate::game::rendering::Renderer;
use crate::game::Game;
use crate::resources::model::Model;
use crate::window::Window;
use crate::resources::loader::ResourceLoader;
use crate::resources::texture::Texture;
use crate::resources::material::Material;
use crate::game::object::Object;

pub struct Runner {
    window: Window,
    game: Game,
    loader: ResourceLoader,
    context: SharedContext,
}

impl Runner {
    pub fn new(window: Window, game: Game, loader: ResourceLoader, context: SharedContext) -> Runner {
        Runner {
            window,
            game,
            loader,
            context,
        }
    }

    pub fn run(self) {
        let Runner {
            window,
            game,
            mut loader,
            context,
        } = self;

        let proxy = window.get_event_proxy();

        window.center();
        let (display, event_loop) = window.take();
        let game = Arc::new(RwLock::new(game));
        let lock = game.clone();

        let mut path = std::env::current_dir().unwrap();
        path.push(std::path::Path::new("assets/assets.json"));

        loader.load_file(&display, std::fs::File::open(std::path::Path::new(path.as_path())).unwrap());

        std::thread::spawn(move || {
            let mut last_update = std::time::Instant::now();
            loop {
                let mut game = { lock.read().unwrap().clone() };
                game.update();
                {
                    *lock.write().unwrap() = game;
                }
                let frame_time = last_update.elapsed();
                let target_time = std::time::Duration::from_secs_f64(1.0 / 60.0);
                std::thread::sleep(
                    if let Some(duration) = target_time.checked_sub(frame_time) {
                        duration
                    } else {
                        std::time::Duration::new(0, 0)
                    },
                );
                last_update = std::time::Instant::now();
            }
        });

        let lock = game.clone();
        let mut renderer = Renderer::new();
        event_loop.run(
            move |event: GlutinEvent<Event>,
                  _target: &EventLoopWindowTarget<Event>,
                  flow: &mut ControlFlow| {
                match event {
                    GlutinEvent::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        *flow = ControlFlow::Exit;
                    }
                    GlutinEvent::NewEvents(_) => {}
                    GlutinEvent::EventsCleared => {
                        let game = { lock.read().unwrap().clone() };
                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                        renderer.render(&game, &mut target, &display, context.clone());
                        target.finish().unwrap();
                        proxy.send_event(Event::PostRender).unwrap();
                    }
                    _ => {}
                }
            },
        );
    }
}
