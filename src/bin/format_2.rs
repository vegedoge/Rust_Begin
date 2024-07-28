// precise format
fn main(){
    //fill in strs
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {:width$}!", "x", width = 5);

    //fill in numbers
    println!("Hello {:5}!", 5);
    println!("Hello {:05}!", 5);
    println!("Hello {:05}!", -5);

    //alignment
    // left
    println!("Hello {:<5}!", "x");
    // right
    println!("Hello {:>5}!", "x");
    // mid
    println!("Hello {:^5}!", "x");

    // precision
    let pi = 3.1415;
    println!("Pi: {:.2}", pi);
    println!("Pi: {:+.}", pi);
    println!("Pi: {:.1$}", pi, 3);

    let s = "My name is Cesar";
    println!("{:.8}", s);

    // base
    println!("{:#b}", 27);
    println!("{:#x}", 27);

    println!(" Hello {{World}} ");
    println!(" Hello \"{{World}}\" ")

}