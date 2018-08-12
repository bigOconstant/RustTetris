

#[derive(Serialize,Deserialize,Debug)]
pub struct GameDataS {
    pub fps: i32,
    pub width:i32,
    pub height:i32
}

use std;
use serde_json;

pub fn load_data(location: String)-> GameDataS {
  
    
    let contents = std::fs::read_to_string(location)
        .expect("Something went wrong reading this file");

    let json: GameDataS = serde_json::from_str(&contents).unwrap();

    return json;
}
