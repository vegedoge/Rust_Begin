// life span intro
fn main(){
    let string1 = String::from("Life span");
    let string2 = "hello !";
    let longer = longest(string1.as_str(), string2);
    println!("The longer string is {longer}");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
{
    if s1.len() > s2.len(){
        s1
    }else{
        s2
    }
}