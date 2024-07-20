use std::fmt::Display;

// constrained trait
struct Pair<T>{
    x: T,
    y: T,
}

impl <T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self { x, y}
    }
}
impl <T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest number is x={}", self.x);
        }
        else{
            println!("The largest number is y={}", self.y);
        }
    }
}

fn main(){
    let pair = Pair::new(3.2, 4.9);
    pair.cmp_display();
}

