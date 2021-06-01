#[warn(unused_imports)]
// 这里定义了一个枚举类型，v4和v6称作枚举的成员 variants
#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn init_enums() {
    // 创建枚举成员实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: "127.0.0.1".to_string(),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: "::1".to_string(),
    };
    println!("v4 and v6 kind is ：{:?},{:?}", home.kind, loopback.kind);
    println!(
        "v4 and v6 address is ：{},{}",
        home.address, loopback.address
    )
}

struct QuitMessage;
#[warn(dead_code)]

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

impl Message {
    fn call(&self, str: &IpAddrKind) {
        println!("this is {:#?} message call", str)
    }
}

pub fn route(ip_type: IpAddrKind) {
    let y = ChangeColorMessage(32, 32, 32);
    let a = Message::ChangeColor(y);
    a.call(&ip_type);
    let som_string = Some("sd".to_string());
    let som_copy = som_string.clone();
    match som_string {
        Some(som_string) => println!("{}", som_string),
        None => println!("foo is none"),
    }

    let absent_number: Option<i32> = None;

    let foo = Some("23");
    match foo {
        Some(i) => println!("{}", i),
        None => println!("foo is none"),
    }
    println!("ip type is :{:#?}", ip_type);
    println!("som_string is :{:#?}", som_copy);
    println!("absent_number is :{:#?}", absent_number);
    println!("foo is :{:#?}", foo)
}
