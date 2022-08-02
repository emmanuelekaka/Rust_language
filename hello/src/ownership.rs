fn cop(a:i32,b:i32){
    println!("{}", a+b);
}
fn main() {
    // let x = 1;
    // let y = 2;
    // {
    // a is only with in the scope of curl braces
    // let x = 10;
    // }
    // println!("{}", x);
    // Dealing with strings
    let s = String::from("Emma");
    // This results in an error so the solution is borrowing
    // let ss = s;
    let ss = &s;
    println!("{}", ss);
    println!("{}", s);



    let a=12;
    let b = 14;
    cop(a,b);
}
