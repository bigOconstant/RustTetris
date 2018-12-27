#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

//use std::sync::Mutex;

   lazy_static! {
   pub static ref GAMEDATA: data::GameDataS = data::load_data("src/assets/data.json".to_string());
}

extern crate sdl2;
extern crate rand;


pub mod data;
pub mod game;
pub mod board;
pub mod startmenu;
pub mod level;
pub mod bdimentions;
pub mod gameend;

pub mod player;


