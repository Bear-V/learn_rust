// 创建公有的结构体和枚举
#[allow(dead_code)]
mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: "this is sum".to_string(),
            }
        }
    }
}

#[test]
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("params");
    println!("{:#?}", meal);
    meal.toast = "change params".to_string();
    println!("{:?}", &meal.toast)
}
