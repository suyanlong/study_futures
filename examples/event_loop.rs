extern crate futures;
extern crate tokio_core;

use std::thread;
use futures::Future;
use futures::sync::oneshot;
use tokio_core::reactor::Core;
use futures::{future, Async, Poll};
fn main() {
    println!("hello world!");
    let mut event_loop = Core::new().unwrap();
    let kk = future::ok::<u32, u32>(3);
    let ret = event_loop.run(kk);
    println!("ret = {:?}", ret);

    let kk = Sleep {
        name: "suyanlong".to_string(),
        sleep_time: 32,
    };
    let ret = event_loop.run(kk);
    println!("ret = {:?}", ret);
}

struct Sleep {
    name: String,
    sleep_time: u64,
}

///简单的实现一个Futrue trait
impl Future for Sleep {
    type Item = String;
    type Error = ::std::io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("===-=-=-=-");
        //thread::park(); poll里面不能有阻塞的操作。
        Ok(Async::Ready(self.name.clone()))
    }
}
