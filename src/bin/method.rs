// test for method in rust
#![allow(unused)]
struct Circle
{
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle
{
    fn new(x:f32, y:f32, radius:f32) -> Circle
    {
        Circle
        {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f32
    {
        std::f32::consts::PI * (self.radius * self.radius)
    }
}

fn main()
{
    let circle1 = Circle::new(5.0 , 11.0, 3.0);
    println!("The area of circle1 is {}", circle1.area());
}

