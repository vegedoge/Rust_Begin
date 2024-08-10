// practice for Trait, Generics and Trait Obj
trait Shape {
    fn area(&self) -> f64;
    fn display(&self);
}


// Create Shapes
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn display(&self) {
        println!("Circle with radius {}", self.radius);
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn display(&self) {
        println!("Rectangle with width {} and height {}", self.width, self.height);
    }
}

// Create Canvas
struct Canvas {
    shapes: Vec<Box<dyn Shape>>,
}

impl Canvas {
    fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn draw(&self) {
        for shape in &self.shapes {
            shape.display();
            println!("Area: {}", shape.area());
        }
    }

}
fn main() {
    let circle = Circle { radius: 5.0};
    let rectangle = Rectangle { width: 5.0, height: 10.0};

    let mut canvas = Canvas::new();

    canvas.add_shape(Box::new(circle));
    canvas.draw();

    canvas.add_shape(Box::new(rectangle));
    canvas.draw();
}