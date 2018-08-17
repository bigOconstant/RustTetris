


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate sdl2;



pub mod data;
pub mod game;   

//Ignore mod test it was auto generated
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
