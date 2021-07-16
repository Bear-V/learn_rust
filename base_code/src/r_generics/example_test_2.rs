#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[test]
fn test_point() {
    let both_int_float = Point { x: 32, y: 3.0 };
    println!("{:#?}", both_int_float)
}
