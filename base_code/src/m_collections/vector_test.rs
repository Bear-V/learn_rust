// vector 向量 顺序容器
#[test]
fn normal_vector_test() {
    let _v: Vec<i32> = Vec::new();
    // or
    let _other_v = vec![123, 213, 123];
}
// Rustacean 开发者的名字 Crustacean
// changed vector value and read vector value
#[test]
fn vector_value() {
    let mut v = Vec::new();
    v.push(56);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let third = &v[2];

    if let Some(third) = v.get(2) {
        println!("success");
        println!("{}", &third);
    }

    println!("{}", third);

    for i in &mut v {
        println!("{}", &i);
        *i += 50;
    }

    // 定义枚举可以在 vector 中存放不同的数据类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", v);
}
