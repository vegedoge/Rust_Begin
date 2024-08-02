fn main() {
    let s = String::new();

    let update_string = || println!("{}", s);
    
    exec(update_string);
    exec1(update_string);
    exec2(update_string);

    let f = factory;
    let f_ed = f(6);
    println!("{:?}", f_ed(25));
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    
    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}