use std::thread;
use std::time::Duration;

#[test]
fn one() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the the main thread! ", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

#[test]
fn two() {
    let v = vec![1, 2, 3];

    // move 是重点，需要获取 v 的所有权， 避免提前杀死
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
