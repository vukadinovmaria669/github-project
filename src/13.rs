use std::ops::Add;

fn main() {
    let mut sum = 0i32;
    for i in 1..10 {
        sum += i;
    }
    println!("{}", sum);
}
