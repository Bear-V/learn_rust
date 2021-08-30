use std::result::Result::Ok;

#[test]
fn one() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("color is {}", color);
    } else if is_tuesday {
        println!("tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple");
        } else {
            println!("using orange");
        }
    } else {
        println!("using blue as the background color");
    }
}

#[test]
fn two() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[test]
fn third() {
    let v = vec!['a', 'b', 'c'];

    for (i, v) in v.iter().enumerate() {
        println!("{} is at index {}", v, i);
    }
}

#[test]
fn four() {
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    //
    // if let x = 5 {
    //     println!("{}", 5);
    // }
}

// 匹配字面值
#[test]
fn five() {
    let x = 1;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("_"),
    }
}

// 匹配命名变量
#[test]
fn six() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched , y = {:?}", y),
        _ => println!("default case x = {:?}", x),
    }

    println!("at the end x = {:?} y = {:?}", x, y);
}

// 多个模式
#[test]
fn seven() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }
}

// 通过 ..= 匹配值的范围
#[test]
fn eight() {
    let x = 5;
    match x {
        // 1 ..= 5 等价 1 ｜ 2 ｜3 ｜4 ｜5
        1..=5 => println!("one through five"),
        _ => println!("something"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// 解构并分解值
struct Point {
    x: i32,
    y: i32,
}

// 解构结构体
#[test]
fn nine() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on the x,y axis at ({},{})", x, y),
    }
}

// 解构枚举

#[test]
fn ten() {
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// 解构嵌套的结构体和枚举
#[test]
fn eleven() {
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

//  _ 忽略值

// .. 忽略剩余值
#[test]
fn twelve() {
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 1, z: 2 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// 匹配守卫提供的额外条件
// 匹配守卫 match guard 是一个指定于 match 分支模式之后的额外 if 条件
// 它也必须满足才能选择此分支
// 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况

#[test]
fn thirteen() {
    // 1
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => println!("none"),
    }

    // 2
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // 3
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ 绑定
// at 运算符 允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
#[test]
fn fourteen() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
