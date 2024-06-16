// This is a test for rust ownership

fn main(){
    let _s1 = give_ownership();

    let s2 = String::from("Hello");

    let _s3 = takes_and_give_back(s2);
    // println!("{}", s2);  s2 is now dropped
}

fn give_ownership() -> String{
    let s = String::from("hello");
    s
}

fn takes_and_give_back(a_string: String) -> String{
    a_string
}