pub fn immutable_var() {
    let x = 5;
    //这里会报错
    // x = 6;
    // {:?} 美化打印代码
    println!("the immutable var of x is: {:?}", x)
}

pub fn var() {
    let mut x = 5;
    x = x + 1;
    println!("the var of x is :{:?}", x)
}

pub fn constant() {
    // 常量强制定义类型 建议大写
    const X: u8 = 1;
    // 此处会报错
    // x = 2;
    println!("{}", X)
}

pub fn shadowing() {
    // 这里在运行的时候会报警告⚠️
    let _x = 1; // 未引用的值 会被建议_开头
    let x = "1";
    // 强转&str为u32
    let x = 2 * x.to_string().parse::<u32>().unwrap();
    println!("shadowing x is :{:?}", x);

    // 隐藏
    let spaces = "    ";
    let spaces = spaces.len();
    println!("the spaces length:{}", spaces);

    // 这样会报错
    // let mut spaces = "";
    // spaces = spaces.len()
}
