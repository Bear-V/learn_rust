use std::sync::mpsc;
use std::thread;

#[test]
fn one() {
    // mpsc multiple producer, single consumer 多个生产者单个消费者
    // tx transmitter 发送者 。 rx receiver 消费者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
}
