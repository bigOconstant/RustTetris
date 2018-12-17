extern crate sdl2;
use ::GAMEDATA;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;


pub struct Level {

}


pub struct BDimentions {
  unit_size:i32,
  left:i32,
  top:i32,
}



impl Level {

    pub fn new() ->Level{
        Level{}
    }


    pub fn draw_level(&self,canvas: &mut sdl2::render::WindowCanvas,level_string:String) {
        // Draw some retangles here!    
        let dimentions:BDimentions = BDimentions{
            unit_size:GAMEDATA.height / 20,
            left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 20)),
            top:GAMEDATA.height - (19*(GAMEDATA.height / 20))
        };

        
        let target = Rect::new((dimentions.left - dimentions.unit_size *5) as i32,dimentions.top,(dimentions.unit_size *4) as u32,(dimentions.unit_size * 3) as u32);
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);

         let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/zig_font.ttf", 128).unwrap();
        let surface = font.render(&level_string).blended(green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        

        let TextureQuery { width, height, .. } = texture.query();

        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();

        
    }

}
