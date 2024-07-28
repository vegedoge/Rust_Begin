// fromat Display & Debug
struct Person{
    name: String,
    age: u8,
}

// use new type to package external type
struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Person{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hello, my name is {}, aged {}",
            self.name, self.age
        )
    }
}

// impl display for self-defined type
impl fmt::Display for Array{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array is: {:?}", self.0)
    }
}

fn main(){
    let p = Person{
        name: "Blake".to_string(),
        age: 23,
    };
    println!("{}", p);

    let array = Array(vec![1, 3, 5]);
    println!("{}", array);

    // change order of placeholders
    println!("sequence: {1}{0}", 1, 2);
    println!("{1} is {0}, {0} is {1}", "Alice", "Bob");

    // assign names for arguments
    // names must be put behind anonymous ones
    println!("a is {a}, b is {}, c is {c}","Bob", a = "Alice", c = "Kevin");
    
}