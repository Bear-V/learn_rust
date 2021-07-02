#[test]
fn example_fn() {
    let v = vec![12, 123, 424];
    v[99];
}

#[allow(dead_code)]
fn throw_panic() {
    panic!("this is a panic")
}
