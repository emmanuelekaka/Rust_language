// enums can take in values (tupples, structs, single type value)
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}
struct Point{
    x:u32,
    y:u32,
}
fn main() {
    let u = Direction::Down({x:23, y:45});
    println!("{}",);
}
