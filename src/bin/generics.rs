// Study for generics
#[derive(Debug)]
struct Point<T, U>{
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point { x: self.x, y: other.y }
    }
}

// specialization for certain type
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]){
    println!("{:?}", arr);
}

fn main(){
    let p1 = Point {x: 5.0, y: 10.0};
    let p2 = Point{x: "Hello", y: 'C'};
    let p3 = p1.mixup(p2);

    let p4: Point<f32, f32> = Point {x: 3.0, y: 4.0};

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p4's distance from origin is {}", p4.distance_from_origin());

    let array: [i32; 3] = [1, 2, 3];
    let array2: [f32; 2] = [2.54, 3.16];
    display_array(array);
    display_array(array2); 
}