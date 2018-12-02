#[macro_use]
extern crate lazy_static;
//use std::sync::Mutex;

extern crate gamedata;


fn main() {


     let game = gamedata::game::Game {
        
     };

    println!("Printing out game data first");
   // println!("data width:{}",GAMEDATABLAH.width);
    game.run_game();
}
