#[derive(Debug)]
struct Rectangle {
    width: isize,
    height: isize,
}

impl Rectangle {
    fn area(&self) -> isize {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: isize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn calculation() {
    let rect1 = Rectangle {
        width: 200,
        height: 300,
    };
    println!("result is :{}", rect1.area());
    let rust_1st = rect1.can_hold(&Rectangle {
        width: 300,
        height: 300,
    });
    println!("result_1st is :{}", rust_1st);
    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq)
}
