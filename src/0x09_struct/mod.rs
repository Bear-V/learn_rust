mod fun_struct;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

fn init_user() {
    let user1 = User {
        username: String::from("pirate"),
        email: String::from("pirate@abc.com"),
        sign_in_count: 1,
        active: false,
    };
    // .. 语法指定了剩余未显示设置值字段应有与给定实例对应字段的值
    let _user2 = User {
        email: "".to_string(),
        username: "".to_string(),
        ..user1
    };
}

fn assignment_user() {
    let mut user1 = User {
        username: "".to_string(),
        email: "".to_string(),
        sign_in_count: 0,
        active: false,
    };
    user1.email = "abc@123.com".to_string();
    // println!("User is {}", user1) 这里不可以直接打印 struct
    println!("User is {:#?}", user1)
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn init_struct() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

pub fn init_struct_fun() {
    init_user();
    assignment_user();
    build_user("123".to_string(), "zzx".to_string());
    init_struct();
    fun_struct::calculation();
}
