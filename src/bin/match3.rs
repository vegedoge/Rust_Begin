// all mode match
#[derive(Debug)]
struct Point
{
    x: i32,
    y: i32,
}

enum Message
{
    Hello{id: i32},
}
fn main()
{
    let p = Point{x: 0, y: 7};
    match p
    {
        Point{x, y:0} => println!("y on the x-axis at {}", x),
        Point{x: 0, y} => println!("x on the y-axis at {}", y),
        Point{x, y} => println!("x, y at ({}, {})", x, y)
    }
    // test for @
    let msg = Message::Hello { id: 5 };
    match msg
    {
        Message::Hello { id: id_variable @ 4..=7 } =>
        {
            println!("found id is {}", id_variable);
        },
        Message::Hello { id: 10..=12 } =>
        {
            println!("found is in range 10-12");
        }
        Message::Hello { id } =>
        {
            println!("found id is {}", id);
        }

    }
    // test for @ deconstruct 
    let p @ Point{x: px, y: py} = Point{x: 10, y: 27};
    println!("px is {}, py is {}", px, py);
    println!("p is {:?}", p);
    
    let point = Point{x: 5, y: 10};
    if let p @ Point {x: 10, y} = point
    {
        println!("x is 10 and y is {} in {:?}",y ,p);
    }
    else 
    {
        println!("x is not 10");
    }





    
}