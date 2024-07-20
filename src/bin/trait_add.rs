// impl for my own add
use std::ops::Add;

#[derive(Debug)]

struct Point<T: Add<T, Output = T>>{
    x: T,
    y: T,
}

impl <T: Add<T, Output = T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T>{
        Point { x: self.x + p.x, y: self.y + p.y }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main(){
    let p1 = Point{x: 1.2f32, y: 3.2f32};
    let p2 = Point{x: 2.1f32, y: 0.1f32};
    println!("p1 + p2 = {:?}", add(p1, p2));

    let p3 = Point{x: 3i32, y: 5i32};
    let p4 = Point{x: 4i32, y: 2i32};
    println!("P3 + P4 = {:?}", add(p3, p4));
}

