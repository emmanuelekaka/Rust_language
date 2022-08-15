fn main() {
    let n = Some(12);
    // Looks only at one binding not many so be carefull while using it.
    if let Some(i) = n {
        println!("{}", i);
    }
    // while let is also present
    // Type casting
    // Type casting to a character 255 as char
    let f = 94.4565;
    let i = f as u8;
    let c = i as char;
    println!("float: {}\nInteger: {}\n Character: {}", f, i, c);
}
