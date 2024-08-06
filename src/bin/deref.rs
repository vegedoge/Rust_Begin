// Trait: Deref for intelligent pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = MyBox::new(5);

    // struct without Deref will cause error
    assert_eq!(5, *x);
}