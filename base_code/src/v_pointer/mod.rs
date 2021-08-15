#[cfg(test)]
mod pointer_test {
    use crate::v_pointer::List::{Cons, Nil};

    #[test]
    fn one() {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    #[test]
    fn two() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", list);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
