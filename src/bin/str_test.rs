// in rust, '' is for char
// "" is for str list
// in rust, char is unicoded with 4 bytes, while
// it's 1 byte in C++
use std::mem::size_of_val;
fn main(){
    let cl = 'a';
    assert_eq!(size_of_val(&cl), 4);
    
    let cnl = '赢';
    assert_eq!(size_of_val(&cnl), 4);

    let str_list = "win";

    println!("the length of -win- is {}", size_of_val(&str_list));
    println!("length confirm success");

    let unit = ();
    println!("the size of unit () is {}", size_of_val(& unit));
}