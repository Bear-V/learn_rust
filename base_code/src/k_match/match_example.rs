#[allow(dead_code)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(stat) => {
            println!("State quarter from : {:?}", stat);
            25
        }
    }
}

#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[test]
fn cents_test() {
    let coin = Coin::Penny;
    let stat_coin = Coin::Quarter(UsState::Alabama);
    let result = value_in_cents(coin);
    println!("result is : {}", result);
    let stat_result = value_in_cents(stat_coin);
    println!("stat result is : {:?}", stat_result);
    println!("============");
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let _none = plus_one(None);
    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("third"),
        5 => println!("five"),
        // 咩有匹配到就啥都不做
        _ => (),
    }
}
