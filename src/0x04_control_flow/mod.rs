pub fn if_function() {
    let num = 3;
    if num < 3 {
        println!("small")
    } else {
        println!("big")
    }
}

pub fn let_if_fun() {
    let condition = true;
    let b = if condition { 5 } else { 8 };
    println!("this true number is :{}", b)
}
