pub fn slice_string(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{},{}", i, &item);
        if item == b' ' {
            return i;
        }
    }
    // 这里在上面寻找到 item == b' '的时候就已经返回了 结束了方法 所以下列不会打印
    println!("this s is :{}", s);

    s.len()
}

pub fn slice_1st() {
    let mut s = String::from("hello world");
    let word = slice_string(&s); // word is 5
    s.clear(); // 这里清空了字符串 释放掉了
    println!("被清空的字符串,{}", s);
    println!("还存在的word，{}", word);
}

pub fn string_slice() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{}", hello);
    println!("{}", world);
}
