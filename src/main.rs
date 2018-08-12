
extern crate gamedata;

fn main(){
   
    let data = gamedata::data::load_data("/home/wolf/Documents/rust/gamedata/src/assets/data.json".to_string());

    println!("Frames per second:{}",data.fps);
    println!("Width:{}",data.width);
    println!("Height:{}",data.height);
}
