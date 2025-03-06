use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 2);
    map.insert("banana", 3);
    map.insert("cherry", 4);
    for (fruit, count) in map {
        println!("{}: {}", fruit, count);
    }
}
