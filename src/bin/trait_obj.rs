// practice for trait obj
trait Draw{
    fn draw(&self);
}

struct Circle;
struct Rectangle;

impl Draw for Circle{
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Draw for Rectangle{
    fn draw(&self) {
        println!("Drawing a Rectangle");
    }
}

fn draw_shapes(shapes: Vec<Box<dyn Draw>>){
    for shape in shapes{
        shape.draw();
    }
}

fn main(){
    let circle = Box::new(Circle);
    let rect = Box::new(Rectangle);

    let shapes: Vec<Box<dyn Draw>> = vec![circle, rect];

    draw_shapes(shapes);
}
