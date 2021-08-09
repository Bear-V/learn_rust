fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用

fn test_fn() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

#[test]
fn test() {
    // 这里的x 在其作用域中结束被删除 ，最下面的r的 借用已经死亡
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //
    //     println!("r : {}", r)
    // }
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);

    test_fn()
}
