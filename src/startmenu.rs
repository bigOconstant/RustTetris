extern crate sdl2;
use bdimentions;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::image::{LoadTexture, InitFlag};
use std::path::Path;

pub struct Startmenu {


}
// Here we need to draw our start menu with the navas object
impl Startmenu {
    pub fn new() -> Startmenu{

        Startmenu{}
    }

    pub fn draw_menu(&self,canvas: &mut sdl2::render::WindowCanvas,context: &sdl2::Sdl){
        

        let dimentions = bdimentions::Bdimentions::new();

        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(191, 191, 191);
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/Roboto-Regular.ttf", 128).unwrap();
        let  banner = "Press Enter to Play".to_string();
        let surface = font.render(&banner).blended(white).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let texture_creator = canvas.texture_creator();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new((dimentions.left - dimentions.unit_size *4) as i32,dimentions.top+dimentions.unit_size * 3,width/2 as u32,height/2 as u32);
        canvas.set_draw_color(yellow);



         let texturewolf = texture_creator.load_texture(Path::new("src/assets/wolf.png")).unwrap();

        let frames_per_anim = 6;
        let mut sprite_tile_size = (100,70);

        let mut source_rect_0 = Rect::new(25, 155, sprite_tile_size.0, sprite_tile_size.1);
        let mut dest_rect_0 = Rect::new(10, 10, sprite_tile_size.0, sprite_tile_size.1);
       
        let mut timer = context.timer().unwrap();

        let ticks = timer.ticks() as i32;
        let seconds = ticks / 250;

        let sprites = seconds % 6;

          match sprites {
            0=> source_rect_0.set_x(25),
            1 => {source_rect_0.set_x(25 + (100))},
            2 => {source_rect_0.set_x(35 + (100 * 2))},
            3 => {source_rect_0.set_x(47 + (100 * 3))},
            4 => {source_rect_0.set_x(55 + (100 * 4))},
            5 => {source_rect_0.set_x(57 + (100 * 5))},
            _ => {source_rect_0.set_x(54 + (100 * 5))},
        } ;
     

        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.copy_ex(&texturewolf, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
       
    }

}
