fn main(){
    //  variables
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    println!("a = {:?}, b = {:?}", a, b);
    assert_eq!(a, b);
}