// 引用 HashMap
use std::collections::HashMap;

#[test]
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", &map);
    let value = map.get(&1).unwrap();
    println!("{}", value)
}
