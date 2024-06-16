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
    let c = '中';
    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // assert adress equal
    assert_eq!(get_addr(r1),get_addr(r2));
    println!("& and ref works the same");

}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}


