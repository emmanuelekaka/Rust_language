fn main() {
    let x = loop {
        break 10;
    };
    println!("x: {}", x);
    let mut k = 1;

    while k < 12 {
        println!("{}", k);
        k += 1;
    }
    // For loops in rust
    let a = vec![12, 23, 45, 66];
    println!("{}", a[1]);
    for i in a {
        println!("{}", i);
    }
    // Ranges in rust
    // 1..=4 includes 1..4 without
    for i in 1..=4 {
        println!("{}", i);
    }
}
