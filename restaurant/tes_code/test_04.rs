// 设计公有枚举，使其所有成员公有
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[test]
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}
