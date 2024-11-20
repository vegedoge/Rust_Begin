use std::sync::mpsc::channel;
use std::thread;

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering::*};

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>
}

impl<T> SpinLock<T> {
    pub fn lock(&self) {
        while self.locked.compare_exchange(false, true, Acquire, Relaxed).is_err() {
            std::thread::yield_now();
        }

        self.locked.store(false, Release);
    }
}

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        let result = some_expensive_operation();

        sender.send(result).unwrap();
    });

    println!("{}", receiver.recv().unwrap());
}