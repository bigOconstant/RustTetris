extern crate sdl2;
use std;
use std::path::Path;

use data::GameDataS;
//use std::env;
//use std::fs;
//use std::io::prelude::*;

//use std::process;
//use  std::thread;
use sdl2::rect::{Rect};

use sdl2::rect::Point;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Physics {

     pub gravity_speed:f32,
     pub  gravity:f32,
     pub  speedx:f32,
     pub speedy:f32,
     pub position_x: f32,
    pub position_y: f32,
    pub jump:bool,

}



pub struct Game {
    pub game_data:GameDataS,
    //    pub gravityspeed:f32
    

}

impl Game {

   
 
    pub fn run_game(&self){

        fn hit_bottom(bottom:i32,Physic: &mut Physics){
            //   println!("Changing value of gr:{} to 8.0",*gr);
            if Physic.position_y > bottom as f32 {
                Physic.position_y = bottom as f32;
                Physic.gravity_speed = 0.0;
                Physic.jump = false;
            }
           
            
        }

        let jump = false;
        fn accelerate(n:f32,gravity: &mut Physics){
            
            gravity.gravity = n;
        }
         let bottom = self.game_data.height - 200;

        fn new_pos(bottom:i32,Physic: &mut Physics){
          //  println!("Jump=:{}",Physic.jump);
            if Physic.jump {
                Physic.gravity += 0.014;
             /*   if Physic.position_y <= bottom as f32{
                    Physic.gravity = 0.0;
                    Physic.position_y = bottom as f32;
                    Physic.speedy = 0.0;
                    Physic.gravity = 0.0;
                    Physic.jump = false;
                    
                }*/
            }
            Physic.gravity_speed += Physic.gravity;
            Physic.position_x += Physic.speedx;
            Physic.position_y += Physic.speedy +Physic.gravity_speed;
            hit_bottom(bottom,Physic);
        }
        

        let mut Solder_P:Physics = Physics {
            gravity_speed:0.0,
            gravity: 0.0,
            speedx:0.0,
            speedy:0.0,
            position_x:117.0,
            position_y:(self.game_data.height - 200) as f32,
            jump:false
        };



    //    hitBottom(&mut gravity);
        println!("Now gravity = {}",Solder_P.gravity);

        
    println!("Running Game Loop");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("SDL2",self.game_data.width as u32,self.game_data.height as u32)
            .position_centered().build().unwrap();

        if self.game_data.fullscreen {
            window.set_fullscreen(sdl2::video::FullscreenType::True);
        }
       
        let icon = sdl2::surface::Surface::load_bmp(Path::new("./src/assets/icon.bmp")).unwrap();
    window.set_icon(icon);
    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

   
        
    canvas.set_draw_color(sdl2::pixels::Color::RGBA(9,7,58,1));
    canvas.set_draw_color(sdl2::pixels::Color::RGB(66, 71, 79));

    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("./src/assets/characters.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();
    let frames_per_anim = 4;
    let sprite_tile_size = (32,32);

    let frame_delay = 1000/ self.game_data.fps as i32;

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(self.game_data.width - 20,bottom));
   
   
    let mut running = true;
    let mut direction = false;
   
    while running {
       
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    if Solder_P.position_x > 0.0 {
                        Solder_P.position_x = Solder_P.position_x -5.0;
                    }
                    direction = true;
                },
                
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    if Solder_P.position_x < (self.game_data.width -100) as f32{
                        Solder_P.position_x =  (Solder_P.position_x +5.0) as f32;
                    }
                    direction = false;
   
                 },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                  
                  if !Solder_P.jump { 
                  Solder_P.jump = true;
                      accelerate(-0.44,&mut Solder_P);
                  }
                  
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                  
                },
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;
        new_pos(bottom,&mut Solder_P);
        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x( Solder_P.position_x as i32);
        dest_rect_2.set_y(Solder_P.position_y as i32);

        canvas.clear();

        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, direction, false).unwrap();
        
        canvas.present();
        
        let frame_time = timer.ticks() as i32;
        
        let frame_time = frame_time - ticks;
        
        if frame_delay > frame_time{
           let sleeptime = (frame_delay - frame_time) as u64;
           std::thread::sleep(Duration::from_millis(sleeptime));
        }

    }

        
    }
}

