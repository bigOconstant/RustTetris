extern crate sdl2;
use std;
use std::path::Path;
////use data::GameDataS;
use board;

use startmenu;
use ::GAMEDATA;



use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;


/*
pub fps: i32,
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,

*/

pub struct Game {
   
}

impl Game {

    pub fn run_game(&self) {

        //Expect 16 by 9 ratio multiplying by 3 for scaling
        //48 by 27
        //Mid point is 24. 
        // Each square is 1 by 1
       

        let sectionwidth = GAMEDATA.width / 3;
        let sectionheight = GAMEDATA.height / 12;

        let midpoint = GAMEDATA.width /2;

        let unitSize = GAMEDATA.height / 27;
    
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem
            .window(
                "Tetris by Caleb",
                GAMEDATA.width as u32,
               GAMEDATA.height as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        if GAMEDATA.fullscreen {
            window.set_fullscreen(sdl2::video::FullscreenType::True);
        }

        let icon = sdl2::surface::Surface::load_bmp(Path::new("src/assets/tetris.bmp")).unwrap();
        window.set_icon(icon);
        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let mut timer = sdl_context.timer().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let frame_delay = 1000 / GAMEDATA.fps as i32;

        let mut running = true;
        let mut board = board::Board::new();
        let startmenu = startmenu::Startmenu::new();
           // bb.initboard();
           let mut start = true;
           let mut playing = false;

        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        running = false;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        if playing {
                            board.drop_piece();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        if playing {
                            board.down_left();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        if playing {
                            board = board::Board::new();
                        }
                    }

                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        if playing {
                            board.down_right();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        if playing {
                            board.switch_piece();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Return),
                        ..
                    } => {
                       if start{
                        start = !start;
                        playing = !playing;
                       }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => {
                        if playing {
                            board.up_key();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => {
                        if playing {
                         board.down_key();
                         }
                    }
                    _ => {}
                }
            }


            let ticks = timer.ticks() as i32;
           

          canvas.clear();
          canvas.set_draw_color(sdl2::pixels::Color::RGB(38, 37, 37));      
          canvas.clear();
          
          if !start{       
              board.draw_board(&mut canvas,ticks);
             
          }else{
              startmenu.draw_menu(&mut canvas);
          }
          canvas.present();




 
            let frame_time = timer.ticks() as i32;

            let frame_time = frame_time - ticks;

            if frame_delay > frame_time {
                let sleeptime = (frame_delay - frame_time) as u64;
                std::thread::sleep(Duration::from_millis(sleeptime));
            }
        }
    }
}
