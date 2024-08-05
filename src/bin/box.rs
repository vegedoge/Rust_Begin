// practice for intelligent pointer Box
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!( "This is button {}", self.id);
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("This is Selection {}", self.id);
    }
}
fn main() {
    let arr = [0;1000];
    let arr1 = arr;

    // both arr and arr1 are assigned to stack, no ownership trans
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0;1000]);
    let arr1 = arr;
    
    // arr is droped cause it's on heap
    println!("{:?}", arr1.len());

    // put different types into one vec
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 3})];

    for e in elems {
        e.draw();
    }
    // * for pointer
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    println!("The sum of deref-ed arr is {}", **first + **second);

    let s = gen_static_str();
    println!("{}", s);
}

// generate a str with lifetime static
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("Hello, static lifetime");

    Box::leak(s.into_boxed_str())
}