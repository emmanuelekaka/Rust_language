fn main() {
    println!("Hello Every one");
    // Dealing with strings
    let s = String::from("String");
    let ss = String::from("Docker!");
    println!("{}", s);
    println!("{}", ss);
    // slicing Strings by accessing its memory
    println!("{:?}", &ss[1..4]);
    // concanating strings using references
    let m = s + " " + &ss;
    println!("{}", m);
}
