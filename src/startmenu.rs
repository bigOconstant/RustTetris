extern crate sdl2;
use BDimentions;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;



pub struct Startmenu {


}
// Here we need to draw our start menu with the navas object
impl Startmenu {
    pub fn new() -> Startmenu{

        Startmenu{}
    }

    pub fn draw_menu(&self,canvas: &mut sdl2::render::WindowCanvas){
        

        let dimentions = BDimentions::BDimentions::new();

        
        
        let yellow: sdl2::pixels::Color = sdl2::pixels::Color::RGB(226, 244, 66);
        let green: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 179, 0);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/Roboto-Regular.ttf", 128).unwrap();
        let mut score_string = "Press Enter to Play".to_string();
        //score_string.push_str(&score.to_string());
        let surface = font.render(&score_string).blended(green).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();


        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new((dimentions.left - dimentions.unit_size *4) as i32,dimentions.top+dimentions.unit_size * 3,width/2 as u32,height/2 as u32);
        canvas.set_draw_color(yellow);
        canvas.copy(&texture, None, Some(target)).unwrap();
       
    }

}
