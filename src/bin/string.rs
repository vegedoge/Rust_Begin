// string and slice
fn main(){
    let mut s = String::from("Hello");

    // push for char
    s.push(',');
    println!("Now s is: {}", s);

    // push_str for string
    s.push_str("rust!");
    println!("Nos s is: {}", s);

    let mut replaced = s.replace("rust", "Rust");
    dbg!(&replaced);

    replaced.replace_range(7..10, "UST");
    println!("s after replacement: {}", s);

    let mut string_remove = String::from("这是Chinese测试");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    println!("After removal: {}", string_remove);
    
    // recovery
    string_remove.insert(0, '这');

    // remove second Chinese character
    string_remove.remove(3);
    println!("After another removal: {}", string_remove);

    string_remove.truncate(10);
    println!("After another truncate: {}", string_remove);

    // add
    let string_append = String::from("Good night,");
    let added = String::from(" Sunday");

    let result = string_append + &added;
    let mut result = result + "!";
    result += "!";

    println!("String cated is: {}", result);

    // ownership moves with add op
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let s_sum = s1 + "-" + &s2 + "-" + &s3;
    
    // s1 is illegal now
    println!("s_sum is: {}", s_sum);
    println!("s2 is: {}", s2);

    // format!
    let s_sum2 = format!("{} {}!", s2, s3);
    println!("fromated s_sum2 is: {}", s_sum2);


}