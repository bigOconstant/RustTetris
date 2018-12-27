#![windows_subsystem = "windows"]// Keep a console from popping up on windows
extern crate lazy_static;
extern crate rusttetris;


fn main() {

     let game = rusttetris::game::Game {};
    game.run_game();
}
