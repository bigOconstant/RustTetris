


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


pub mod data;

//Ignore mod test it was auto generated
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
