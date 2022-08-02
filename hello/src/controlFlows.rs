fn main() {
    let m = 12;
    // rust has if, if else, and else
    if m > 12 {
        println!("{}", m);
    } else {
        println!("Its less");
    }
    // if statements are expressions in rust
    let n = true;
    // below an if shd relate with boolean not numerical like other languages
    // Also types should be the same
    let value = if n { 45 } else { 34 };
    println!("{}", value);

    // loops for ever
    // loop {
    //     println!("Die hard");
    // }
    // controlled loops
    let mut constant = 0;
    loop {
        println!("{}", constant);
        if constant == 12 {
            break;
        }
        // rust doesn't support ++, -- operator
        constant += 1;
    }
    // loop labeling
    'a: loop {
        println!("This is loop  a");
        'b: loop {
            println!("This is loop b");
            'c: loop {
                println!("This is loop c");
                break;
            }
            break;
        }
        break;
    }
}
