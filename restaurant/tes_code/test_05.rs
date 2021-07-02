// 使用 use 关键字将名称引入作用域

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("test use keyword")
        }
    }
}

// 创建别名
use crate::front_of_house::hosting as h;

#[test]
pub fn eat_at_restaurant() {
    h::add_to_waitlist();
    h::add_to_waitlist();
    h::add_to_waitlist();
}
