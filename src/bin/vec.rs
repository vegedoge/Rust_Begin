// vector

trait IpAddr{
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4{
    fn display(&self) {
        println!("ipv4 {:?}", self.0);
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6 {:?}", self.0);
    }
}

#[derive(Debug)]
struct Person{
    name: String,
    age: u32,
}

impl Person{
    fn new(name: String, age: u32) -> Person {
        Person{name, age}
    }
}

fn main(){
    let v = vec![1, 2, 3, 4, 5, 6];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    for i in &v{
        println!("{i}");
    }

    let address: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("192.168.1.108".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in address{
        ip.display();
    }

    // capacity
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);    // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100);        // 调整 v 的容量，至少要有 100 的容量
    println!("Vector(reserve) 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.shrink_to_fit();     // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!("Vector(shrink_to_fit) 长度是: {}, 容量是: {}", v.len(), v.capacity());

    // sort
    let mut people = vec![
        Person::new("Geroge".to_string(), 32),
        Person::new("Adam".to_string(), 50),
        Person::new("Walter".to_string(), 41),
    ];

    people.sort_unstable_by(|a,b| a.age.cmp(&b.age));
    println!("{:?}", people);
    
}
