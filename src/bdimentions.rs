pub struct BDimentions{
  pub unit_size:i32,
  pub left:i32,
  pub right:i32,
  pub bottom:i32,
  pub top:i32,
  pub width:i32,
  pub height:i32,
}


impl BDimentions {
    pub fn new() -> BDimentions {
        let width = 1280;
        let height = 720;


        BDimentions{
        unit_size:height / 22,
        left:(width /2) - (5 * (height / 22)),
        right:(width /2) + (5 * (height / 22)),
        bottom:height - (1*(height / 22)),
        top:height - (21*(height / 22)),
        width:width,
        height:height
      }
    }
}
