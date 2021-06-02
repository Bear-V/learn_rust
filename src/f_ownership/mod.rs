pub fn _string() {
    let _s = String::from("hello");
}

pub fn change_string() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str("world!");
    println!("new String s:{}", s)
}

pub fn _move_fun() {
    let x = 5;
    let y = x;
    println!("x move in y:{}", y)
}

pub fn move_string_fun() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 point :{},{}", "s1", s2)
}

pub fn clone_string() {
    let x = String::from("test");
    let y = x.clone();

    println!("x is {}, y is {}", x, y)
}

pub fn copy_fun() {
    // 这里证明基本类型是可以直接进行copy的
    let x = 1;
    let y = x;
    println!("{},{}", x, y);
    let x = false;
    let y = x;
    println!("{},{}", x, y);
    let x = 'z';
    let y = x;
    println!("{},{}", x, y);
    let _x = "xzx";
}

// 赋值
pub fn assignment_fun() {
    // s 进入作用域
    let s = String::from("hello");

    take_ownership(s); //s 的值移动到函数里
                       // s 没有了

    let x = 5; // x 进入作用域
    make_copy(x); // x进入函数  但是固定大小类型是 copy

    println!("the last value is x :{}", x)
} // 这里 x 也没有了

fn take_ownership(some_string: String) {
    // 进入作用域
    println!("{}", some_string);
} // 移除 调用 drop方法

fn make_copy(some_integer: i32) {
    // 进入作用域
    println!("{}", some_integer);
} // 移除 调用 drop 方法

pub fn move_ownership() {
    let s = give_ownership(); // 将返回值赋值给s 移交了所有权

    let ss = String::from("hello"); //ss 进入作用域

    let sss = take_and_give_back(ss); // ss没有了 , 返回值赋值给了 sss
    println!("s is:{},sss is:{}", s, sss)
} // 这里s和sss都没有了

fn give_ownership() -> String {
    let str = String::from("new hello");
    str
} // str 返回给调用的函数

fn take_and_give_back(a_string: String) -> String {
    a_string
}

pub fn move_tup() {
    let s = String::from("this is new life");

    let (ss, len) = calculate_length(s);

    println!("the length of {} is {}", ss, len)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

pub fn new_move_tup() {
    let s = String::from("hello");
    let len = new_calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn new_calculate_length(s: &str) -> usize {
    s.len()
}

pub fn quote() {
    fn calculate_length(s: &str) -> usize {
        s.len()
    }
    let s = String::from("this is new s");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}
