// Trait: Deref for intelligent pointer
use std::ops::Deref;
struct MyBox<T> {
    v: T
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox{v: x}
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use::std::ops::DerefMut;

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn display(s: &str) {
    println!("{}", s);
}

fn display_mut(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}

fn main() {
    let x = MyBox::new(5);

    // struct without Deref will cause error
    assert_eq!(5, *x);

    // auto Deref: only be done to &var
    let s = Box::new(String::from("Hello, Rust"));
    display(&s);
    
    let mut s2 = MyBox::new(String::from("CPP, "));
    display_mut(&mut s2);
    let _s3 = s.to_string();
}