#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

//use std::sync::Mutex;

    lazy_static! {
   pub static ref GAMEDATA: data::GameDataS = data::load_data("assets/data.json".to_string());
}

extern crate sdl2;
extern crate rand;


pub mod data;
pub mod game;
pub mod board;
pub mod startmenu;

pub mod player;

//Ignore mod test it was auto generated
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
