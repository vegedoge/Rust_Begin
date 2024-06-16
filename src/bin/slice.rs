fn main(){
    let s = String::from("Hello World!");

    let _slice = &s[..5];
    let slice = &s[0..5];
    println!("slice is {}", slice);

    let len = s.len();
    let back = &s[6..len];
    println!("back is {}", back);

    let all = &s[..];
    println!("all is {}", all);

    let word = first_word(&s);
    println!("First word is {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}