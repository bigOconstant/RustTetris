#[macro_use]
extern crate lazy_static;
//use std::sync::Mutex;

extern crate rusttetris;


fn main() {


     let game = rusttetris::game::Game {};
    game.run_game();
}
