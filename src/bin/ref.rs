// test of reference and pointer
fn main(){
    let mut s = String::from("Hello");

    let p1 = &s;
    let p2 = &s;

    println!("*p1:{}, *p2{}", *p1, *p2);

    let p3 = &mut s;
    p3.push_str(", world!");

    println!("s is {}, *p3 is {}", *p3, *p3);
    // s is not allowed to be visited here
    // we know that prinln! s needs a ref to s
}