// 互斥器一次只允许一个线程访问数据
// 1。 再使用数据之前尝试获取锁
// 2。 粗粝玩被互斥器锁保护的数据之后，必须解锁数据，这样其他县城才能够获取锁
// RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性

use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn one() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

#[test]
fn third() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
