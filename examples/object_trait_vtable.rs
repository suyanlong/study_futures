extern crate futures;

use std::io;
use std::time::Duration;
use futures::prelude::*;
use futures::future::Map;

// A future is actually a trait implementation, so we can generically take a
// future of any integer and return back a future that will resolve to that
// value plus 10 more.
//
// Note here that like iterators, we're returning the `Map` combinator in
// the futures crate, not a boxed abstraction. This is a zero-cost
// construction of a future.
fn add_ten<F>(future: F) -> Map<F, fn(i32) -> i32>
              where
                  F: Future<Item = i32>,
{
    fn add(a: i32) -> i32 {
        a + 10
    }
    future.map(add)
}

// Not only can we modify one future, but we can even compose them together!
// Here we have a function which takes two futures as input, and returns a
// future that will calculate the sum of their two values.
//
// Above we saw a direct return value of the `Map` combinator, but
// performance isn't always critical and sometimes it's more ergonomic to
// return a trait object like we do here. Note though that there's only one
// allocation here, not any for the intermediate futures.
fn add<'a, A, B>(a: A, b: B) -> Box<Future<Item = i32, Error = A::Error> + 'a>
                 where
                     A: Future<Item = i32> + 'a,
                     B: Future<Item = i32, Error = A::Error> + 'a,
{
    Box::new(a.join(b).map(|(a, b)| a + b))
}

fn main() {
    println!("hello wrold");
}
