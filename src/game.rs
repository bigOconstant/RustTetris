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

pub struct Game {
pub game_data:GameDataS,

}

impl Game {

 
    pub fn run_game(&self){
    println!("Running Game Loop");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2",self.game_data.width as u32,self.game_data.height as u32)
            .position_centered().build().unwrap();

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
    dest_rect_2.center_on(Point::new(self.game_data.width - 20,self.game_data.height - 100));
   
    let mut position_x = 117;
    let mut running = true;
    let mut direction = false;
   
    while running {
       
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    if position_x > 0 {
                        position_x = position_x -5;
                    }
                    direction = true;
                },
                
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    if position_x < self.game_data.width -100{
                        position_x =  position_x +5;
                    }
                    direction = false;
   
                 },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                  
                  
                  
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                  
                },
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;
    
        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x( position_x);

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

