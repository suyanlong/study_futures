#[macro_use]
extern crate lazy_static;
use std::cell::Cell;
//use std::cell::RefCell;
//use std::string::ToString;
//use std::thread;
use std::collections::HashMap;
//学习一下rust宏的展开。需要掌握一下两个命令。
//rustc --pretty expanded -Z unstable-options -o ./log lazy_static_var.rs
//cargo expand --example lazy_static_var

//thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
//static mut map: RefCell<HashMap<u32, u32>> = {
//    let mut kk = HashMap::with_capacity(100);
//    kk.insert(100, 100);
//    RefCell::new(kk)
//};

lazy_static! {
    static ref BUFFER:Cell<Vec<u8>>= Cell::new((0..65537).collect());
}

fn main() {
    //    println!("{:?}", String::from_utf8(BUFFER).unwrap());
    //    let kk = BUFFER.len();
    //    BUFFER.push(2); //狗屁还是不行。
}
