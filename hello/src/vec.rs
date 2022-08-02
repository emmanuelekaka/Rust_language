fn main() {
    let v = vec![4, 5, 6, 7, 8, 9, 10, 7, 6, 4];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {}", i, r);
    }
}
// Operation to count occurance of a given value in a vector
fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
