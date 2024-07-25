// life span intro
struct Book<'a>{
    title: &'a str,
    author: &'a str,
}

enum Status<'a>{
    Active(&'a str),
    Inactive
}

fn main(){
    let string1 = String::from("Life span");
    let string2 = "hello !";
    let longer = longest(string1.as_str(), string2);
    println!("The longer string is {longer}");

    // life span for struct and enum
    let title = String::from("Rust Programming");
    let author = "John";

    let book = Book{
        title: &title,
        author: author,
    };

    println!("{} by {}", book.title, book.author);

    let status_message = String::from("system online");
    let status = Status::Active(&status_message);

    if let Status::Active(msg) = status{
        println!("status: {msg}");
    }

}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
{
    if s1.len() > s2.len(){
        s1
    }else{
        s2
    }
}