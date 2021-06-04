pub fn int() {
    const A: i8 = -1;
    const B: u8 = 1;
    println!("{}", A + B as i8)
}

pub fn math() {
    // 加法
    let sum = 1 + 2;
    // 减法
    let difference = 95.3 - 93 as f64;
    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    println!(
        "sum: {},difference: {},product: {},quotient: {},remainder: {}",
        sum, difference, product, quotient, remainder
    )
}

pub fn different_string() {
    let c = 'z'; // char值 z
    let s = "zzzz"; // 指针s
    let z = s.to_string();
    println!("char z: {}", c);
    println!("string &s: {}", s);
    println!("string s: {}", z)
}

pub fn tup_type() {
    let tup = (55, 53.4, 'z', "this is string");
    let (x, _y, _z, _) = tup;
    println!("x in tup: {}", x);

    println!("other x in tup :{}", tup.0)
}

pub fn array_type() {
    let _arr = [0, 1, 2, 3, 4, 5];
}

// 手动定义一个每一个元素都相同的数组
pub fn same_array() {
    let arr = [3; 5];
    let value = arr[2];
    println!("a values same of array : {:?}", arr);
    println!("value : {}", value)
}
