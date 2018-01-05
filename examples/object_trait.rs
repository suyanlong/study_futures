trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        let value = format!("u8: {}", *self);
        println!("{}", value);
        value
    }
}

impl Foo for String {
    fn method(&self) -> String {
        let value = format!("string: {}", *self);
        println!("{}", value);
        value
    }
}

fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = "Hello".to_string();
    do_something(&x);
    let y = 8u8;
    do_something(&y);
}
