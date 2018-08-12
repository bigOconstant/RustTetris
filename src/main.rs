
extern crate gamedata;

fn main(){
    let address_data = "/home/wolf/Documents/rust/gamedata/src/assets/data.json";
    let address_data = address_data.to_string();
    let data = gamedata::data::load_data(address_data);

    println!("Frames per second:{}",data.fps);
    println!("Width:{}",data.width);
    println!("Height:{}",data.height);
}
