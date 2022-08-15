fn main() {
    let x = 12;
    match x {
        1 => println!("{}", x),
        23 => println!("{}", x),
        12 => println!("Its 12"),
        32 => println!("{}", x),
        _ => println!("{}", x),
    }
    // more complexity on match
    let m = 12;
    match m {
        1 => println!("{}", m),
        7 | 14 | 12 => println!("{}", m),
        2..=5 => println!("{}", m),
        _ => println!("{}", m),
    }
    // With tupples
    let k = (12, 5);
    match k {
        (12, y) => println!("{}", y),
        (x, 4) => println!("{}", x),
        _ => println!("None"),
    }
    // More on tupples
    let tupple = (13, 5);
    match tupple {
        (x, y) if x == y => println!("{}", y),
        (x, y) if x + y == 0 => println!("{}", x),
        (x, _) if x == 0 => println!("{}", x),
        _ => println!("None"),
    }
    // Pattern Matching/ find where it lies.
    let pattern = 5;
    match pattern {
        k @ 1..=5 => println!("k: {}", k),
        k @ 6..=10 => println!("k: {}", k),
        _ => println!("None"),
    }
    // inline of above
    let kin = 10;
    let patt = match kin {
        k @ 1..=5 => k,
        k @ 6..=100 => k,
        _ => 0,
    };
    println!("{}", patt);
}
