#![feature(backtrace)]
#![feature(box_syntax)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate labyrinth_cgmath;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod game;
pub mod resources;
pub mod runner;
pub mod window;
pub mod error;

use crate::error::LabyrinthErrorBase;

pub type LabyrinthResult<T> = Result<T, Box<dyn LabyrinthErrorBase>>;