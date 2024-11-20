use std::thread;
use std::sync::Mutex;

static V: Mutex<i32> = Mutex::new(0);

fn count(delta: i32) {
    for _ in 0..10000 {
        *V.lock().unwrap() += delta;
    }
}

fn local_count(delta: i32, v: &Mutex<i32>) {
    for _ in 0..10000 {
        *v.lock().unwrap() += delta
    }
}

fn main() {
    let t1 = thread::spawn(|| count(1));
    let t2 = thread::spawn(|| count(-1));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("{}", V.lock().unwrap());

    // local mutex

    // allocate v on heap to prevent cleaning
    let v = Box::new(Mutex::new(0));

    // let t3 = thread::spawn(|| local_count(1, &v));
    // let t4 = thread::spawn(|| local_count(-1, &v));

    thread::scope(|s| {
        s.spawn(|| local_count(1, &v));
        s.spawn(|| local_count(-1, &v));
    });

    println!("local ref with scope: {}", v.lock().unwrap());
}