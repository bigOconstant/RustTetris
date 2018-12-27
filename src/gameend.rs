extern crate sdl2;
use bdimentions;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;



pub struct Gameend {}
impl Gameend {
    pub fn new() -> Gameend{
        Gameend{}
    }

    pub fn draw_end(&self,canvas: &mut sdl2::render::WindowCanvas){
     
        let dimentions = bdimentions::Bdimentions::new();
        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(191, 191, 191);
        let texture_creator = canvas.texture_creator();

        // Load a font
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font("src/assets/Roboto-Regular.ttf", 128).unwrap();
        let score_string = "Game Over".to_string();

        let surface = font.render(&score_string).blended(white).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();


        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new((dimentions.left +4) as i32,dimentions.top+dimentions.unit_size * 4,width/2-2 as u32,height/2 as u32);
        let wider_target_border = Rect::new((dimentions.left ) as i32,dimentions.top+dimentions.unit_size * 4-2,322 ,104 );
        
        
        let wider_target = Rect::new((dimentions.left +3) as i32,dimentions.top+dimentions.unit_size * 4,316 ,100 );
        //canvas.set_draw_color(yellow);

        canvas.set_draw_color(white);  
        canvas.fill_rect(wider_target_border).expect("could not fill rectangle");
        canvas.set_draw_color(sdl2::pixels::Color::RGB(38, 37, 37));  
        canvas.fill_rect(wider_target).expect("could not fill rectangle");
        canvas.set_draw_color(white);
        canvas.copy(&texture, None, Some(target)).unwrap();
       
    }

}
