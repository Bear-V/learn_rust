// 将模块分割进不同的文件
mod front_of_house;

// 创建别名
use crate::front_of_house::hosting as h;

#[test]
pub fn eat_at_restaurant() {
    h::add_to_waitlist();
    h::add_to_waitlist();
    h::add_to_waitlist();
}
