#![feature(duration_from_micros)]
extern crate futures;
extern crate futures_cpupool;
extern crate httparse;
extern crate hyper;
extern crate parking_lot;
extern crate rand;
extern crate serde;

use parking_lot::Mutex;
use std::io;
use futures::{BoxFuture, Future};
use rand::Rng;
use std::sync::Arc;
use std::collections::HashMap;
use futures::sync::oneshot;
use std::collections::VecDeque;

use futures::future::FutureResult;
use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, NewService, Request, Response, Service};
use futures_cpupool::{CpuFuture, CpuPool};

use std::thread;
use std::time::Duration;
use std::ops::Deref;
use std::ops::DerefMut;

impl NewService for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Server;
    fn new_service(&self) -> io::Result<Server> {
        Ok(self.clone())
    }
}

struct Message {
    id: i32,
    body: String,
}

#[derive(Clone)]
struct Server {
    hash_map: Arc<Mutex<VecDeque<(String, oneshot::Sender<String>)>>>,
    cpu_pool: CpuPool,
}

use futures::Stream;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        println!("method: {:?}, path: {:?}, ", req.method(), req.path());
        let map = self.hash_map.clone();
        self.cpu_pool
            .spawn_fn(move || {
                let (tx, rx) = oneshot::channel();
                //println!("-----post --{:?}----", req.slice((1, 100)));
                let random_id = rand::thread_rng().gen_range(1, 50000);
                println!("-----call -----{:?}-", random_id);
                {
                    map.lock().push_back((random_id.to_string(), tx));
                }

                //这个时候已经可以了啊，满足我们的需求了。之前这里有sleep，作为超时，不能这样做，需要用futures类型的超时，才可以！！！！
                rx.map(|item| {
                    println!("=={:?}==", item);
                    let mut res = Response::new();
                    res.set_body(item);
                    res
                }).map_err(|_| hyper::Error::Timeout)
                    .boxed()
            })
            .boxed()
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let arc_hash_map = Arc::new(Mutex::new(VecDeque::new()));
    let hash_map = arc_hash_map.clone();

    let _ = thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_micros(10));
            let mut map = arc_hash_map.lock();
            let map: &mut VecDeque<(String, oneshot::Sender<String>)> = map.deref_mut();
            map.pop_front().map(|(k, v)| {
                println!("----k = {:?}", k);
                v.send(k);
            });
        }
    });

    let server = Server {
        hash_map: hash_map,
        cpu_pool: CpuPool::new(4),
    };
    let server = Http::new().bind(&addr, server).unwrap();
    println!(
        "Listening on http://{} with 1 thread.",
        server.local_addr().unwrap()
    );
    server.run().unwrap();
}
