extern crate gamedata;

fn main(){
    let  game = gamedata::game::Game {
        game_data:
        gamedata::data::load_data("./src/assets/data.json".to_string())
    };        

    game.run_game();
}
