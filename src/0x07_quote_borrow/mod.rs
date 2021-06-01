pub fn mutable_quote() {
    let mut s = String::from("hello");

    let str = change(&mut s);
    println!("&mut string str is {:?}", str)
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(",world");
    some_string
}
