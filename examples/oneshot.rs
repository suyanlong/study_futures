extern crate futures;

use std::thread;
use futures::Future;
use futures::sync::oneshot;

fn expensive_computation() -> u32 {
    // ...
    200
}

fn main() {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        thread::sleep_ms(10000);
        tx.send(expensive_computation()).unwrap();
    });

    let rx = rx.map(|x| x + 3);
    let result = rx.wait().unwrap();
    assert_eq!(result, 203);
}
