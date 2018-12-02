
extern crate sdl2;
use ::GAMEDATA;
use player;
//use sdl2::rect::Rect;
use sdl2::rect::Point;

#[derive(Clone)]
pub struct Piece{
  pub rect:sdl2::rect::Rect,
  pub color:sdl2::pixels::Color,
  pub occupied:bool,

}
pub struct BDimentions{
  midpoint:i32,
  unit_size:i32,
  left:i32,
  right:i32,
  bottom:i32,
  top:i32,
}

pub struct Board {
  pub  bmatrix:  Vec<Vec<Piece>>,
  pub preview_matrix: Vec<Vec<Piece>>,
  pub players: Vec<player::Player>,
}

impl Board {

    pub fn new() ->Board {
      let BLUE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(91, 89, 89);
      let light_black: sdl2::pixels::Color = sdl2::pixels::Color::RGB(38, 37, 37);
      
      let mut p = player::Player::new();
      let mut p2 = player::Player::new();
      let mut player_list :Vec<player::Player> = Vec::new();
      player_list.push(p2);
      player_list.push(p);

      let dimentions:BDimentions = BDimentions{
        midpoint: GAMEDATA.width /2,
        unit_size:GAMEDATA.height / 20,
        left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 20)),
        right:(GAMEDATA.width /2) + (5 * (GAMEDATA.height / 20)),
        bottom:GAMEDATA.height - (1*(GAMEDATA.height / 20)),
        top:GAMEDATA.height - (19*(GAMEDATA.height / 20))
      };

      let clr : sdl2::pixels::Color = BLUE;

      let mut Board_Pieces : Vec<Vec<Piece>> = Vec::new();
            for j in 0..18{
          let mut Row: Vec<Piece> =  Vec::new();
          for i in 0..10{
            
              let positioned_retangle : sdl2::rect::Rect = sdl2::rect::Rect::new(2+dimentions.left +
                      (i * dimentions.unit_size), dimentions.top+2+(dimentions.unit_size * j),
                      (dimentions.unit_size -3) as u32,(dimentions.unit_size -3) as u32);
            let p:Piece = Piece{rect:positioned_retangle,color:clr,occupied:false};
            Row.push(p);
          }
          Board_Pieces.push(Row);
        }

      let mut Preview_Board_Pieces: Vec<Vec<Piece>> = Vec::new();
        for j in 0..4{
          let mut Row: Vec<Piece> = Vec::new();
          for i in 0..4{
            let positioned_retangle_right : sdl2::rect::Rect = sdl2::rect::Rect::new(2+dimentions.right + dimentions.unit_size +
                      (i * dimentions.unit_size), dimentions.top+2+(dimentions.unit_size * j),
                      (dimentions.unit_size -3) as u32,(dimentions.unit_size -3) as u32);
                      Row.push(Piece{rect:positioned_retangle_right,color:light_black,occupied:false})
          }
          Preview_Board_Pieces.push(Row);

        }


      Board{bmatrix: Board_Pieces,preview_matrix:Preview_Board_Pieces,players:player_list}
      
    }

    fn draw_pieces(&self,canvas: &mut sdl2::render::WindowCanvas){
          for i in &self.bmatrix{
            for j in i{
              canvas.set_draw_color(j.color);
              canvas.fill_rect(j.rect);
          }
          for i in &self.preview_matrix{
            for j in i{
              canvas.set_draw_color(j.color);
              canvas.fill_rect(j.rect);
          }
        }
    }}


    pub fn down_key(&mut self){
      
      let mut cloned_player = self.players[0].clone();
      cloned_player.decr();
      
       if self.do_i_fit(&cloned_player) {
          self.delete_piece();
        if !self.is_occupied(cloned_player){
          self.players[0].decr();
        }
       }  
    }
    pub fn drop_piece(&mut self) {
       let mut cloned_player = self.players[0].clone();
       cloned_player.decr();
       self.delete_piece();
       if self.do_i_fit(&cloned_player){
         if !self.is_occupied(cloned_player){
           self.down_key();
           self.drop_piece();
         }
       } else{
         return;
       } 

    }

    pub fn up_key(&mut self) { 
      let mut cloned_player = self.players[0].clone();
      cloned_player.incr_state();

      if self.do_i_fit(&cloned_player) {
         self.delete_piece();
       if !self.is_occupied(cloned_player){
          
          self.players[0].incr_state();
       }
      }
    }

    pub fn switch_piece(&mut self) {
        self.players[0] = self.players[1].clone();
        self.players[1] = player::Player::new();
        self.clear_rows();
        self.clear_future_board();

    }

    pub fn down_left(&mut self){
      
     
      let mut cloned_player = self.players[0].clone();
      cloned_player.left();
      if self.do_i_fit(&cloned_player){
         self.delete_piece();
        if !self.is_occupied(cloned_player){
          self.players[0].left();
        }
      }
    }

    pub fn down_right(&mut self){
      
      
      let mut cloned_player = self.players[0].clone();
      cloned_player.right();
      if self.do_i_fit(&cloned_player){
          self.delete_piece();
        if !self.is_occupied(cloned_player){
         self.players[0].right();
        }
      }
    }

    fn draw_grid(&self,canvas: &mut sdl2::render::WindowCanvas,dimentions: BDimentions){
         //Draw side lines
        canvas.draw_line(Point::new(dimentions.left,dimentions.top),Point::new(dimentions.left,dimentions.bottom));
        canvas.draw_line(Point::new(dimentions.right,dimentions.top),Point::new(dimentions.right,dimentions.bottom));
          //draw top and bottom

        canvas.draw_line(Point::new(dimentions.left,dimentions.top),Point::new(dimentions.right,dimentions.top));         
        canvas.draw_line(Point::new(dimentions.left,dimentions.bottom),Point::new(dimentions.right,dimentions.bottom));


        for i in 1..11 {
         
            canvas.draw_line(Point::new(dimentions.left+(i*dimentions.unit_size),dimentions.top),
                             Point::new(dimentions.left+(i*dimentions.unit_size),dimentions.bottom));
        }

        for i in 1..19{
            canvas.draw_line(Point::new(dimentions.left,dimentions.top + (i*dimentions.unit_size)),
                             Point::new(dimentions.right,dimentions.top +(i*dimentions.unit_size)));          

        }
    }
      fn delete_piece(&mut self) {

        let BLUE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(91, 89, 89);
        let shape = &self.players[0].get_shape();;
        let col = self.players[0].col;
        let row = self.players[0].row;
        let color = self.players[0].color;
        let mut icount = 0;
        for i in 0..shape.len(){
          let mut jcount = 0;
          for j in 0..shape[i].len(){
            
            if shape[i][j] == 1 {
              let colAddress = ((j as i32)+col) as usize;
              let rowAddress = ((i as i32)+row) as usize;

              self.bmatrix[rowAddress][colAddress].color = BLUE;
                  self.bmatrix[rowAddress][colAddress].occupied = false;
              
            }
            jcount = jcount+1;
          }
          icount = icount +1;
        }
    }

    pub fn do_i_fit(&self,play:&player::Player) -> bool {
                  // Edit the board here
      let shape = play.get_shape();
      let col = play.col.clone();
      let row = play.row.clone();

      for i in 0..shape.len(){
        
        for j in 0..shape[i].len(){
          
          if shape[i][j] == 1 {
            let colAddress = ((j as i32)+col) as usize;
            let rowAddress = ((i as i32)+row) as usize;

            if colAddress > 9 || colAddress < 0 {
              return false;
            }
            if rowAddress > 17 || rowAddress < 0 {
              return false;
            }
          } 
        }
      }      
      return true;
    }

    pub fn is_occupied(&self,play:player::Player)->bool{
        // do I fit should have already been called
      let shape = play.get_shape();
      let col = play.col.clone();
      let row = play.row.clone();

            for i in 0..shape.len(){
        
        for j in 0..shape[i].len(){
          
          if shape[i][j] == 1 {
            let colAddress = ((j as i32)+col) as usize;
            let rowAddress = ((i as i32)+row) as usize;

            // Check if position is occupied on board

            if(self.bmatrix[rowAddress][colAddress].occupied) {
              return true
            }
          } 
        }
      }

        return false;
    }

     pub fn draw_a_player(&mut self) {
            // Edit the board here
      let shape = &self.players[0].get_shape();
      let col = self.players[0].col;
      let row = self.players[0].row;
      let color = self.players[0].color;

      for i in 0..shape.len(){
       
        for j in 0..shape[i].len(){
          
          if shape[i][j] == 1 {
            let colAddress = ((j as i32)+col) as usize;
            let rowAddress = ((i as i32)+row) as usize;
             self.bmatrix[rowAddress][colAddress].color = color;
             self.bmatrix[rowAddress][colAddress].occupied = true;   
          }
        }

      }

     }

     pub fn draw_future_player(&mut self){
       let shape = &self.players[1].get_shape();
      let col = self.players[1].col;
      let row = self.players[1].row;
      let color = self.players[1].color;

      for i in 0..shape.len(){
        
        for j in 0..shape[i].len() {
           if shape[i][j] == 1 {
             self.preview_matrix[i][j].color = color;
             self.preview_matrix[i][j].occupied = true;
           }
        }

      }
     }

     pub fn clear_future_board(&mut self){
       //let BLUE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(91, 89, 89);
     
       for i in 0..self.preview_matrix.len(){
        
        for j in 0..self.preview_matrix[i].len() {
           
             self.preview_matrix[i][j].color = sdl2::pixels::Color::RGB(38, 37, 37);
             self.preview_matrix[i][j].occupied = false;
           
        }

      }
     }

    pub fn clear_rows(&mut self){
        let mut val = 0;
       for  i in (0..self.bmatrix.len()) {
         let mut full = true;
         for j in (0..self.bmatrix[i].len()){
           if !self.bmatrix[i][j].occupied  {
            full = false;
            val = i;
           }
         }
          if full {
            for j in (0..self.bmatrix[i].len()){
             self.bmatrix[i][j].occupied = false;
             self.bmatrix[i][j].color = sdl2::pixels::Color::RGB(91, 89, 89);
            }
            for k in (1..i+1).rev(){
              for l in (0..self.bmatrix[k].len()){ 
                self.bmatrix[k][l].color = self.bmatrix[k-1][l].color;
                self.bmatrix[k][l].occupied = self.bmatrix[k-1][l].occupied;
              }
            }
          }
       
     }
    }

    pub fn draw_board(&mut self, canvas: &mut sdl2::render::WindowCanvas,tick:i32) {

        let dimentions:BDimentions = BDimentions{
        midpoint: GAMEDATA.width /2,
        unit_size:GAMEDATA.height / 20,
        left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 20)),
        right:(GAMEDATA.width /2) + (5 * (GAMEDATA.height / 20)),
        bottom:GAMEDATA.height - (1*(GAMEDATA.height / 20)),
        top:GAMEDATA.height - (19*(GAMEDATA.height / 20))
      };

      let WHITE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(187, 190, 193);


       canvas.set_draw_color(WHITE);

       self.draw_grid(canvas,dimentions);
       self.draw_a_player();
       self.draw_future_player();
       self.draw_pieces(canvas);
    }

   
}
