use std::ops::Deref;

use crate::v_pointer::example_box::List::{Cons, Nil};

#[test]
fn one() {
    let b = Box::new(5);
    println!("b = {}", b);
}
// 解引用运算符
#[test]
fn two() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn third() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
#[test]
fn four() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
#[test]
fn five() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 解引用强制多态
#[test]
fn six() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m)
}

fn hello(name: &str) {
    println!("name is {}", name);
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
