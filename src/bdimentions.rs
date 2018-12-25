
use ::GAMEDATA;
pub struct BDimentions{
 // midpoint:i32,
  pub unit_size:i32,
  pub left:i32,
  pub right:i32,
  pub bottom:i32,
  pub top:i32,
}


impl BDimentions {
    pub fn new() -> BDimentions {
        BDimentions{
        //midpoint: GAMEDATA.width /2,
        unit_size:GAMEDATA.height / 22,
        left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 22)),
        right:(GAMEDATA.width /2) + (5 * (GAMEDATA.height / 22)),
        bottom:GAMEDATA.height - (1*(GAMEDATA.height / 22)),
        top:GAMEDATA.height - (21*(GAMEDATA.height / 22))
      }
    }
}
