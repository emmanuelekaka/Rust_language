use std::collections::HashMap;
fn main() {
    // let x = vec![1,3,5,77];
    let mut v = Vec::new();
    v.push(12);
    v.push(23);
    // printing out 1 item
    println!("{}", v[1]);
    // printing out  whole vector
    println!("{:#?}", v);
    // Iterate printing.
    for i in &v {
        println!("{}", i);
    }
    // printing out the lenght and capacity
    println!("length:{} capacity:{}", v.len(), v.capacity());
    // removing the last entered item
    v.pop();
    println!("After popping: {:?}", &v);
    // HASH MAPS
    let mut hashed = HashMap::new();
    hashed.insert(String::from("name"), 12);
    hashed.insert(String::from("age"), 22);
    // println!("{}", hashed["name"]);
    for (v, y) in &hashed {
        println!("{}: {}", v, y);
    }
    match hashed.get(&String::from("name")) {
        Some(&value) => println!("Available is:{}", value),
        _ => println!("Not available"),
    }
    // Removing an element from the hashmap
    hashed.remove(&String::from("age"));

    for (m, r) in &hashed {
        println!("{}: {}", m, r);
    }
}
