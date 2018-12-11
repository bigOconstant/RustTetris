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

pub struct BDimentions{
  midpoint:i32,
  unit_size:i32,
  left:i32,
  right:i32,
  bottom:i32,
  top:i32,
}
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            //println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
           // println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (GAMEDATA.width as i32 - w) / 2;
    let cy = (GAMEDATA.height as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub struct Startmenu {


}
// Here we need to draw our start menu with the navas object
impl Startmenu {
    pub fn new() -> Startmenu{

        Startmenu{}
    }

    pub fn draw_menu(&self,canvas: &mut sdl2::render::WindowCanvas){
        
        let dimentions:BDimentions = BDimentions{
        midpoint: GAMEDATA.width /2,
        unit_size:GAMEDATA.height / 20,
        left:(GAMEDATA.width /2) - (5 * (GAMEDATA.height / 20)),
        right:(GAMEDATA.width /2) + (5 * (GAMEDATA.height / 20)),
        bottom:GAMEDATA.height - (1*(GAMEDATA.height / 20)),
        top:GAMEDATA.height - (19*(GAMEDATA.height / 20))
      };


        let Yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let Green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);
        
        let positioned_retangle : sdl2::rect::Rect = sdl2::rect::Rect::new(dimentions.left +
                      (dimentions.unit_size), dimentions.top+(dimentions.unit_size),
                      (dimentions.unit_size ) as u32,(dimentions.unit_size ) as u32);

        let texture_creator = canvas.texture_creator();

    // Load a font
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font("src/assets/zig_font.ttf", 128).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);
    let surface = font.render("Tetris Press Enter").blended(Green).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let startSurface = font.render("Press Enter").blended(Green).unwrap();
    let texture2 = texture_creator.create_texture_from_surface(&startSurface).unwrap();


    let TextureQuery { width, height, .. } = texture.query();

    let padding = 600;
    let padding2 = 400;

        let target = get_centered_rect(GAMEDATA.width as u32, GAMEDATA.height as u32, GAMEDATA.width as u32 - padding, GAMEDATA.height as u32 - padding);
        //let target2 = get_centered_rect(GAMEDATA.width as u32, (GAMEDATA.height+200) as u32, GAMEDATA.width as u32 - padding2, (GAMEDATA.height+200) as u32 - padding2);
        
        canvas.set_draw_color(Yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();
        //canvas.copy(&texture2, None, Some(target2)).unwrap();
        //canvas.fill_rect(positioned_retangle);
    }

}