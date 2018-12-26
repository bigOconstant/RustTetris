extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use BDimentions;


pub struct Level {

}


impl Level {

    pub fn new() ->Level{
        Level{}
    }

    pub fn draw_level(&self,canvas: &mut sdl2::render::WindowCanvas,level_string:String) {
        // Draw some retangles here!    
        let dimentions = BDimentions::BDimentions::new();

        
        let target = Rect::new((dimentions.left - dimentions.unit_size *5) as i32,dimentions.top,(dimentions.unit_size *4) as u32,(dimentions.unit_size * 3) as u32);
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);

         let texture_creator = canvas.texture_creator();
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/Roboto-Regular.ttf", 128).unwrap();
        let surface = font.render(&level_string).blended(green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();

        
    }

    pub fn draw_score(&self,canvas: &mut sdl2::render::WindowCanvas,score:i32) {
        let dimentions = BDimentions::BDimentions::new();

        
        
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/Roboto-Regular.ttf", 128).unwrap();
        let mut score_string = "Score".to_string();
        //score_string.push_str(&score.to_string());
        let surface = font.render(&score_string).blended(green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new((dimentions.left - dimentions.unit_size *6) as i32,dimentions.top+dimentions.unit_size * 3,width/2 as u32,height/2 as u32);
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();
        // End top half

        

        let surface = font.render(&score.to_string()).blended(green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();

        let mut number_target = Rect::new((dimentions.left - dimentions.unit_size *6) as i32,dimentions.top+dimentions.unit_size * 5,width/3 as u32,height/2 as u32);
        number_target = Rect::new((dimentions.left - dimentions.unit_size *6) as i32,dimentions.top+dimentions.unit_size * 5,width/2 as u32,height/2 as u32);
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(number_target)).unwrap();

    }

}
