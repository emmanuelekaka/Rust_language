use std::mem;
fn main() {
    println!("Hello, world!");
    // none mutable
    let x = 4;
    println!("{x}");
    // mutable
    let mut k = 21;
    println!("{k}");
    k = 55;
    println!("{k}");
    // primitives in rust
    // i8, i32, i64, i128 change to u, String, bool:, i and usize base on computer architecture.
    // floating values f32 and f64
    // Calculations are the same with other programming language, also characters available in single quotes(c: char)
    // tupples
    let t: (i32, char, f32) = (1, 'E', 32.5);
    println!("{}{}", t.0, t.1);
    let (x, y, z) = t; //underscore ignores
    println!("{x}");
    println!("{y}");
    println!("{z}");
    // more on tupples
    let tu = (1, (2, 3, 4));
    // Accessing the rest of elements using debug tray
    println!("{:?}", t);
    // Pretty output line per line
    println!("{:#?}", tu.1);
    // Lists in rust
    let list: [i32; 5] = [1, 2, 4, 6, 8];
    // output all of them
    println!("{:?}", list);
    // determining length of the list
    println!("{}", list.len());
    // Determining the number of bytes occupied by the list
    println!("{}", mem::size_of_val(&list));
    // Determing the number of bytes occupied by 1 value within an array
    println!("{}", mem::size_of_val(&list[0]));
    // accessing a few elements in an array
    let partition = &list[2..4];
    println!("{:?}", partition);
}
