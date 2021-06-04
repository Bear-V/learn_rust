pub fn lambda() {
    let x = 5;
    println!("value x is :{}", x);
    let y = {
        let x = 32;
        x + 1 // 此处注意不能加;结尾
    };
    println!("y is {}", y)
}

pub fn five(x: i8) -> i32 {
    return x as i32;
}

pub fn six(x: i32) -> i64 {
    x as i64 // 不能加分号
}
