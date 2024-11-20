use std::sync::{Arc, Mutex};
use std::thread;

fn count(delta: i32, v: Arc<Mutex<i32>>) {
    for _ in 1..10000 {
        *v.lock().unwrap() += delta;
    }
}

fn main() {
    let v = Arc::new(Mutex::new(0));

    let (v1, v2) = (v.clone(), v.clone());

    let t1 = thread::spawn(|| count(1, v1));
    let t2 = thread::spawn(|| count(-1, v2));

    // dont miss the error in threads
    t1.join().unwrap();
    t2.join().unwrap();

    println!("{}", v.lock().unwrap());
}