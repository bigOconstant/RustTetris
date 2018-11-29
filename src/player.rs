use board::Piece;
extern crate sdl2;
extern crate rand;

use rand::Rng;
#[derive(Clone)]
pub struct Player {
    pub done:bool,
    pub row:i32,
    pub col:i32,
    pub shape:Vec<Vec<i32>>,
    pub shape2:Vec<Vec<i32>>,
    pub shape3:Vec<Vec<i32>>,
    pub shape4:Vec<Vec<i32>>,
    pub color:sdl2::pixels::Color,
    pub shape_state:i32
}

impl Player{

    pub fn decr(&mut self){
        self.row += 1;
    }
    
    pub fn incr(&mut self){
        self.row -= -1;
    }

    pub fn down(&mut self){
        self.decr();
    }
    pub fn left(&mut self){    
        self.col -= 1;
    }
    pub fn right(&mut self){
        self.col += 1;
    }
// Here are the data points
    pub fn new()-> Player{
         let mut rng = rand::thread_rng();
         let shape_number =  rng.gen_range(0, 7);
         let mut clr: sdl2::pixels::Color = sdl2::pixels::Color::RGB(50,205,50);
        println!("Creating random integer here!:{}",shape_number);
        let mut s  = [
            [0,1,0,0],
            [0,1,0,0],
            [0,1,0,0],
            [0,1,0,0]];

        let mut s2 = [
            [0,0,0,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]];

        let mut s3 = [
            [0,1,0,0],
            [0,1,0,0],
            [0,1,0,0],
            [0,1,0,0]];

        let mut s4 = [
            [0,0,0,0],
            [1,1,1,1],
            [0,0,0,0],
            [0,0,0,0]];

        match shape_number {
            0 =>{  
                    clr =  sdl2::pixels::Color::RGB(50,205,50);
                    s  = [
                        [0,1,0,0],
                        [0,1,0,0],
                        [0,1,0,0],
                        [0,1,0,0]];

                    s2 = [
                        [0,0,0,0],
                        [1,1,1,1],
                        [0,0,0,0],
                        [0,0,0,0]];

                    s3 = [
                        [0,1,0,0],
                        [0,1,0,0],
                        [0,1,0,0],
                        [0,1,0,0]];

                    s4 = [
                        [0,0,0,0],
                        [1,1,1,1],
                        [0,0,0,0],
                        [0,0,0,0]];
            },
           
           
            1 =>{  
                clr =  sdl2::pixels::Color::RGB(242, 242, 159);
                s  = [
                [0,1,0,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0]];

                s2 = [
                [0,1,0,0],
                [0,1,1,0],
                [0,1,0,0],
                [0,0,0,0]];

                s3 = [
                [0,0,0,0],
                [1,1,1,0],
                [0,1,0,0],
                [0,0,0,0]];

                s4 = [
                [0,1,0,0],
                [1,1,0,0],
                [0,1,0,0],
                [0,0,0,0]];
            }
            ,
           //J
            2 =>
             {
                clr =  sdl2::pixels::Color::RGB(66, 134, 244);     
                s  = [
                [0,1,0,0],
                [0,1,0,0],
                [1,1,0,0],
                [0,0,0,0]];

            s2 = [
                [1,0,0,0],
                [1,1,1,0],
                [0,0,0,0],
                [0,0,0,0]];

            s3 = [
                [0,1,1,0],
                [0,1,0,0],
                [0,1,0,0],
                [0,0,0,0]];

            s4 = [
                [0,0,0,0],
                [1,1,1,0],
                [0,0,1,0],
                [0,0,0,0]];
            },
        
         3 =>
             {
                 clr =  sdl2::pixels::Color::RGB(244, 134, 66);     
                 s  = [
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0]];

                s2 = [
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0]];

                s3 = [
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0]];

                s4 = [
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0]];
                },

            4 => {
                clr =  sdl2::pixels::Color::RGB(212, 158, 241);     
                  s  = [
                    [0,1,0,0],
                    [0,1,0,0],
                    [0,1,1,0],
                    [0,0,0,0]];

                s2 = [
                    [0,0,0,0],
                    [1,1,1,0],
                    [1,0,0,0],
                    [0,0,0,0]];

                s3 = [
                    [1,1,0,0],
                    [0,1,0,0],
                    [0,1,0,0],
                    [0,0,0,0]];

                s4 = [
                    [0,0,1,0],
                    [1,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0]];
            },
            //S shape shape
            5 => {
                clr =  sdl2::pixels::Color::RGB(255, 204, 255);
                s  = [
                    [0,1,0,0],
                    [0,1,1,0],
                    [0,0,1,0],
                    [0,0,0,0]];

                s2 = [
                    [0,0,0,0],
                    [0,1,1,0],
                    [1,1,0,0],
                    [0,0,0,0]];

                s3 = [
                    [0,1,0,0],
                    [0,1,1,0],
                    [0,0,1,0],
                    [0,0,0,0]];

                s4 = [
                    [0,0,0,0],
                    [0,1,1,0],
                    [1,1,0,0],
                    [0,0,0,0]];

            },
            6 => {
              clr =  sdl2::pixels::Color::RGB(51, 204, 204);
                s  = [
                    [0,0,1,0],
                    [0,1,1,0],
                    [0,1,0,0],
                    [0,0,0,0]];

                s2 = [
                    [0,0,0,0],
                    [1,1,0,0],
                    [0,1,1,0],
                    [0,0,0,0]];

                s3 = [
                    [0,0,1,0],
                    [0,1,1,0],
                    [0,1,0,0],
                    [0,0,0,0]];

                s4 = [
                    [0,0,0,0],
                    [1,1,0,0],
                    [0,1,1,0],
                    [0,0,0,0]];  
            }

           
            _ => {println!("Something Wrong happened")}
        }


        let s: Vec<_> =   s.iter().map(|&e| e.to_vec()).collect();
        let s2: Vec<_> = s2.iter().map(|&e| e.to_vec()).collect();
        let s3: Vec<_> = s3.iter().map(|&e| e.to_vec()).collect();
        let s4: Vec<_> = s4.iter().map(|&e| e.to_vec()).collect();


        Player {
            done:false,
            row:0,
            col:3,
            shape:s,
            shape2:s2,
            shape3:s3,
            shape4:s4,
            color:clr,
            shape_state:0
        }
    }

    pub fn incr_state(&mut self){
        if self.shape_state <3 {
            self.shape_state += 1;
        } else {
            self.shape_state = 0;
        }
    }


    pub fn get_shape(&self)-> Vec<Vec<i32>>{
        match self.shape_state {
            1 =>  return self.shape2.clone(),
            2 =>  return self.shape3.clone(),
            3 => return  self.shape4.clone(),
            _ => return self.shape.clone(),
        }
    }

    pub fn get_col(self)->i32{
        let col = self.col;
        col
    }
    pub fn get_row(self)->i32{
       let row = self.row;
       row
    }

    pub fn get_color(self)->sdl2::pixels::Color{
        let color = self.color;
        color
    }


}