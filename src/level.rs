extern crate sdl2;
use ::GAMEDATA;
use player;
use sdl2::rect::Rect;
use sdl2::rect::Point;


use sdl2::event::Event;

use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Level {

}


pub struct BDimentions {
  midpoint:i32,
  unit_size:i32,
  left:i32,
  right:i32,
  bottom:i32,
  top:i32,
}



impl Level {

     

    pub fn i_worked(&self) {

    }

    pub fn draw_menu(&self,canvas: &mut sdl2::render::WindowCanvas) {
        // Draw some retangles here!    
        let dimentions:BDimentions = BDimentions{
            midpoint: GAMEDATA.width /2,
            unit_size:GAMEDATA.height / 20,
            left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 20)),
            right:(GAMEDATA.width /2) + (5 * (GAMEDATA.height / 20)),
            bottom:GAMEDATA.height - (1*(GAMEDATA.height / 20)),
            top:GAMEDATA.height - (19*(GAMEDATA.height / 20))
        };

        let target = Rect::new(100,dimentions.top,300,200);
        let Yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let Green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);

         let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut font = ttf_context.load_font("src/assets/zig_font.ttf", 128).unwrap();
        let surface = font.render("Level 1").blended(Green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        

        let TextureQuery { width, height, .. } = texture.query();

        canvas.set_draw_color(Yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();

        
    }

}
