use std::collections::HashMap;

#[test]
fn example_hashmap() {
    let txt = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    other_hash_fn();

    other_test_fn();
}

fn other_hash_fn() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 10);

    // 覆盖
    scores.insert("Blue".to_string(), 50);

    // 不存在就插入,存在就不管
    let entry = scores.entry("Yellow".to_string()).or_insert(90);
    println!("检查 插入{:?}", entry);

    println!("{:?}", scores);
}

fn other_test_fn() {
    let field_name = String::from("FA");
    let field_value = String::from("Italy");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", &map);
    // field_value = "213".to_string();
    // println!("{}", field_value);
    // println!("{:?}", &map);
    let option = map.get("FA").unwrap();
    println!("{:?}", option);
}
