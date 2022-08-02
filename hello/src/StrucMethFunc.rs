use std::fmt;
// structs like in c
// #[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn show(&self) {
        println!("{}*{} is {}", self.width, self.height, self.area());
    }
}
// More functionality
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
// By default rust doesn't know how to apply the debug tray or method
fn main() {
    let object = Object {
        width: 12,
        height: 43,
    };
    let new_obj = Object::new(34, 12);
    object.show();
    new_obj.show();
    // Doing work like python but with added addvantage without  debug trays own definition
    println!("{}", object);
    println!("{}", new_obj);
}
