use labyrinth_engine::game;
use labyrinth_engine::window;
use labyrinth_engine::runner;
use labyrinth_engine::resources::loader::ResourceLoader;

fn main() {
    let game = game::Game::new();
    let context = game::context::LabyrinthContext::create();
    let loader = ResourceLoader::new(context.clone());
    let settings = window::WindowSettings::new().with_size(window::WindowSize::new(800, 640));
    let window = window::Window::new(&settings);
    runner::Runner::new(window, game, loader, context.clone()).run();
}
