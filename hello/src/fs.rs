// file system in here
use std::fs::File;
// in a match statement no need for putting ;
fn main() {
    let f = File::open("text.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening the file with error{:?}", error)
        }
    };
}
