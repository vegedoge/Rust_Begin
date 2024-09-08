// course practice on software fundamentals
#[derive(Debug, PartialEq)]
struct Even (i32);

impl Even {
    pub fn new (value: i32) -> Result<Self, &'static str> {
        if value % 2 == 0 {
            Ok(Even(value))
        } else {
            Err("Input number is not even")
        }
    }

    pub fn divide_by_two(&self) -> i32 {
        self.0 / 2
    }

    pub fn multiply(&self, multiplier: i32) -> Even {
        Even(self.0 * multiplier)
    }

}

fn main() {
    let a = Even(8);
    let b = Even::new(7);
    let c = 11;

    let half = a.divide_by_two();
    let mul = a.multiply(c);

    println!("b is {:?}", b);
    println!("{:?} divided by two is {}", a, half);
    println!("{:?} multiplied by {} is {:?}", a, c, mul);


}