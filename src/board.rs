
extern crate sdl2;
use player;
use level;
use bdimentions;
use gameend;


#[derive(Clone)]
pub struct Piece{
  pub rect:sdl2::rect::Rect,
  pub color:sdl2::pixels::Color,
  pub occupied:bool,

}

pub struct Board {
  pub  bmatrix:  Vec<Vec<Piece>>,
  pub preview_matrix: Vec<Vec<Piece>>,
  pub players: Vec<player::Player>,
  pub level_text: level::Level,
  pub rows_cleared:i32,
  pub score:i32,
  pub end:bool,
}

impl Board {

    pub fn new() -> Board {
    
      let blue: sdl2::pixels::Color = sdl2::pixels::Color::RGB(91, 89, 89);
      let light_black: sdl2::pixels::Color = sdl2::pixels::Color::RGB(38, 37, 37);
      let p = player::Player::new();
      let p2 = player::Player::new();
      let mut player_list :Vec<player::Player> = Vec::new();
      player_list.push(p2);
      player_list.push(p);

      let dimentions = bdimentions::Bdimentions::new();
    
      let clr : sdl2::pixels::Color = blue;

      let mut board_pieces : Vec<Vec<Piece>> = Vec::new();
            for j in 0..20{
          let mut row: Vec<Piece> =  Vec::new();
          for i in 0..10{
            
              let positioned_retangle : sdl2::rect::Rect = sdl2::rect::Rect::new(2+dimentions.left +
                      (i * dimentions.unit_size), dimentions.top+2+(dimentions.unit_size * j),
                      (dimentions.unit_size -1) as u32,(dimentions.unit_size -1) as u32);
            let p:Piece = Piece{rect:positioned_retangle,color:clr,occupied:false};
            row.push(p);
          }
          board_pieces.push(row);
        }

      let mut preview_board_pieces: Vec<Vec<Piece>> = Vec::new();
        for j in 0..4{
          let mut row: Vec<Piece> = Vec::new();
          for i in 0..4{
            let positioned_retangle_right : sdl2::rect::Rect = sdl2::rect::Rect::new(2+dimentions.right + dimentions.unit_size +
                      (i * dimentions.unit_size), dimentions.top+2+(dimentions.unit_size * j),
                      (dimentions.unit_size -1) as u32,(dimentions.unit_size -1) as u32);
                      row.push(Piece{rect:positioned_retangle_right,color:light_black,occupied:false})
          }
          preview_board_pieces.push(row);

        }


      Board{bmatrix: board_pieces,preview_matrix:preview_board_pieces,players:player_list,level_text:level::Level::new(),rows_cleared:0,score:0,end:false}
      
    }

    fn draw_pieces(&self,canvas: &mut sdl2::render::WindowCanvas){
          for i in &self.bmatrix{
            for j in i{
              canvas.set_draw_color(j.color);
              canvas.fill_rect(j.rect).expect("could not fill rectangle");
          }
          for i in &self.preview_matrix{
            for j in i{
              canvas.set_draw_color(j.color);
              canvas.fill_rect(j.rect).expect("could not fill rectangle");
          }
        }
    }}


    pub fn down_key(&mut self,point:bool){
      
      let mut cloned_player = self.players[0].clone();
      cloned_player.decr();
      
       if self.do_i_fit(&cloned_player) {
          self.delete_piece();
        if !self.is_occupied(cloned_player){
          self.players[0].decr();
          if point {
            self.score += 1; // Give a point for user dropping to the next square
          }
        }else{
         self.draw_a_player();
         self.switch_piece();// end turn 
        }
       }else{
         self.switch_piece();//end turn
       }  
    }
    pub fn drop_piece(&mut self) {
       let mut cloned_player = self.players[0].clone();
       cloned_player.decr();
       self.delete_piece();
       if self.do_i_fit(&cloned_player){
         if !self.is_occupied(cloned_player){
           self.down_key(true);
           self.drop_piece();
         }else{
           self.draw_a_player();
         }
       } else{
         self.draw_a_player();
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
        }else{
          self.draw_a_player();
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
        }else{
          self.draw_a_player();
        }
      }
    }

      fn delete_piece(&mut self) {
        let blue: sdl2::pixels::Color = sdl2::pixels::Color::RGB(91, 89, 89);
        let shape = &self.players[0].get_shape();;
        let col = self.players[0].col;
        let row = self.players[0].row;
        let mut icount = 0;
        for i in 0..shape.len(){
          let mut jcount = 0;
          for j in 0..shape[i].len(){
            
            if shape[i][j] == 1 {
              let col_address = ((j as i32)+col) as usize;
              let row_address = ((i as i32)+row) as usize;

              self.bmatrix[row_address][col_address].color = blue;
                  self.bmatrix[row_address][col_address].occupied = false;
              
            }
            jcount = jcount+1;
          }
          icount = icount +1;
        }
    }

    pub fn do_i_fit(&self,play:&player::Player) -> bool {
      let shape = play.get_shape();
      let col = play.col.clone();
      let row = play.row.clone();

      for i in 0..shape.len(){
        
        for j in 0..shape[i].len(){
          
          if shape[i][j] == 1 {
            let col_address = ((j as i32)+col) as usize;
            let row_address = ((i as i32)+row) as usize;

            if col_address > 9 {
              return false;
            }
            if row_address > 19  {
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
            let col_address = ((j as i32)+col) as usize;
            let row_address = ((i as i32)+row) as usize;

            // Check if position is occupied on board

            if self.bmatrix[row_address][col_address].occupied {
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
      let first = self.players[0].first_move;
      

      if first { // Test if fits, if it doesn't need to end game
         let cloned_player = self.players[0].clone();
         if self.is_occupied(cloned_player) {// game is over
            self.end = true;

         }
         self.players[0].first_move = false;

      }
      

      if !self.end{
      for i in 0..shape.len(){
       
        for j in 0..shape[i].len(){
          
          if shape[i][j] == 1 {
            let col_address = ((j as i32)+col) as usize;
            let row_address = ((i as i32)+row) as usize;
             self.bmatrix[row_address][col_address].color = color;
             self.bmatrix[row_address][col_address].occupied = true;   
          }
        }

      }
      }
     }

     pub fn draw_future_player(&mut self){
       let shape = &self.players[1].get_shape();
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
       for i in 0..self.preview_matrix.len(){
        
        for j in 0..self.preview_matrix[i].len() { 
             self.preview_matrix[i][j].color = sdl2::pixels::Color::RGB(38, 37, 37);
             self.preview_matrix[i][j].occupied = false;
           
        }

      }
     }

    pub fn clear_rows(&mut self){
        let mut scoring_rows_cleared = 0;
       for  i in 0..self.bmatrix.len() {
         let mut full = true;
         for j in 0..self.bmatrix[i].len(){
           if !self.bmatrix[i][j].occupied  {
            full = false;
           }
         }
          if full {
            scoring_rows_cleared +=1;
            self.rows_cleared +=1;
            for j in 0..self.bmatrix[i].len(){
             self.bmatrix[i][j].occupied = false;
             self.bmatrix[i][j].color = sdl2::pixels::Color::RGB(91, 89, 89);
            }
            for k in (1..i+1).rev(){
              for l in 0..self.bmatrix[k].len() { 
                self.bmatrix[k][l].color = self.bmatrix[k-1][l].color;
                self.bmatrix[k][l].occupied = self.bmatrix[k-1][l].occupied;
              }
            }
          }
     }
      self.calculate_score_of_lines(scoring_rows_cleared);
     //println!("{} lines cleared current score:{}",scoring_rows_cleared,self.score);
    }

    pub fn calculate_level(&mut self)-> i32 {
            let mut earned_level = 1;
            if self.rows_cleared <= 0
            {
                earned_level = 1;
            }
            else if (self.rows_cleared >= 1) && (self.rows_cleared <= 90)
            {
                earned_level = 1 + ((self.rows_cleared - 1) / 10);
            }
            else if self.rows_cleared >= 91
            {
                earned_level = 10;
            }
      earned_level
    }
    pub fn calculate_score_of_lines(&mut self,lines_cleared:i32){
      let level = self.calculate_level();
      let score = match lines_cleared {
        1 =>  40 * (level + 1),
        2 => 100 * (level + 1),
        3 => 300 * (level + 1),
        4 => 1200 * (level + 1),
        _ => 0,     
      };
      self.score += score;
    }

    pub fn draw_board(&mut self, canvas: &mut sdl2::render::WindowCanvas,falling:bool,level_number:i32) {

        if falling && !self.end{
          self.down_key(false);
        }

        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(187, 190, 193);

        let  end_screen = gameend::Gameend::new();
       canvas.set_draw_color(white);

       self.draw_a_player();
       self.draw_future_player();
       self.draw_pieces(canvas);
      // self.draw_score(canvas,self.score);
       let mut level = String::from("Level ");
       level.push_str(&level_number.to_string());
       self.level_text.draw_level(canvas,level);
       self.level_text.draw_score(canvas,self.score);
       
       if self.end {
          end_screen.draw_end(canvas);
        }
    
    }

   
}
