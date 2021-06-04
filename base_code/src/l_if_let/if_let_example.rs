use crate::k_match::match_example::*;

#[test]
fn if_let_fn_test() {
    let some_u8_value = 3u8;
    match some_u8_value {
        3 => println!("third"),
        _ => (),
    }
    // 和上述写法结果一致
    // 当 match 条件只有一个的时候 可以使用if let 来表达
    // if let 是 match 的语法糖
    if let 3 = some_u8_value {
        println!("third");
    }
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("statte quarter from {:?}", state),
        _ => count += 1,
    }

    println!("{}", count);

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("statte quarter from {:?}", state)
    } else {
        count += 1
    }
    println!("{}", count);
}
