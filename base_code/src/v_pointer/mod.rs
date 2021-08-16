use std::ops::Deref;

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

#[cfg(test)]
mod deref_trait {
    use crate::v_pointer::MyBox;

    #[test]
    fn one() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn two() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn third() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
