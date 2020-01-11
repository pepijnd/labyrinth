#![feature(backtrace)]
#![feature(box_syntax)]

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;

use clap::clap_app;

use labyrinth_engine::game;
use labyrinth_engine::resources::loader::ResourceLoader;
use labyrinth_engine::runner;
use labyrinth_engine::window;

use labyrinth_assets::assets::Assets;

mod convert;
mod pack;
mod utils;

use convert::convert;
use pack::pack;


fn main() -> Result<(), Box<dyn labyrinth_engine::error::LabyrinthErrorBase>> {
    let log_lvl = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter(None, log_lvl)
        .init();

    info!("Logging on level {}", log_lvl);

    let matches = clap_app!(labyrinth_app =>
        (version: "0.1.0")
        (author: "Pepijn Dragt")
        (about: "Labyrinth")
        (@subcommand launch =>
            (about: "launch game")
            (@arg file: -f --file +takes_value "file containing game Files")
            (@arg directory: -d --dir +takes_value "Directory containing game Files")
        )
        (@subcommand convert =>
            (about: "convert asset file")
            (@arg input: +required "file to convert")
            (@arg output: +required "output folder")
        )
        (@subcommand pack =>
            (about: "pack asset files")
            (@arg input: +required "input folder")
            (@arg output: +required "output file")
        )
    )
    .get_matches();

    let (target, command) = matches.subcommand();
    match target {
        "pack" => {
            let command = command.unwrap();
            let input = command.value_of("input").unwrap();
            let output = command.value_of("output").unwrap();
            let packed = pack(input)?;
            let packed = bincode::serialize(&packed).unwrap();
            let mut file = std::fs::File::create(output).unwrap();
            file.write_all(&packed).unwrap();
        }
        "convert" => {
            let command = command.unwrap();
            let input = command.value_of("input").unwrap();
            let output = command.value_of("output").unwrap();
            convert(input, output)?;
        }
        "launch" | &_ => {
            let mut game_data: Option<Assets> = None;
            if let Some(command) = command {
                let file = command.value_of("file");
                let dir = command.value_of("directory");
                if let Some(dir) = dir {
                    game_data = pack(dir).ok();
                } else if let Some(file) = file {
                    let data = std::fs::read(file).unwrap();
                    game_data = Some(bincode::deserialize(&data).unwrap());
                }
            }
            if game_data.is_none() {
                let data = std::fs::read("game_data.bin").unwrap();
                game_data = Some(bincode::deserialize(&data).unwrap());
            }
            let game_data = game_data.unwrap();

            let game = game::Game::new();
            let context = game::context::LabyrinthContext::create();
            let mut loader = ResourceLoader::new(context.clone());
            loader.add("main".to_owned(), game_data);
            let settings =
                window::WindowSettings::new().with_size(window::WindowSize::new(1280, 720));
            let window = window::Window::new(&settings);
            runner::Runner::new(window, game, loader, context.clone()).run();
        }
    }

    Ok(())
}
