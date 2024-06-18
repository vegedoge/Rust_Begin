// Trial on Struct
#[derive(Debug)]
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
    email: String,
}

fn build_user(username: String, email: String) -> User{
    User {
        username,
        email,
        active: true,
        sign_in_count: 1 
    }
}

fn main(){
    let username = String::from("ruuuust");
    let email = String::from("xxx@yy.com");
    let user1 = build_user(username, email);

    // struct 更新后，会发生所有权转移
    // user1此时无法使用，但是内部没转移的可以使用
    let user2 = User{
        email: String::from("xxx2@yy.com"),
        ..user1
    };
    println!("user2 active: {}", user2.active);
    println!("user1 email: {}", user1.email);
    println!("user2 username: {}", user2.username);
    println!("user1 sign_in: {}", user1.sign_in_count);
    dbg!(&user2);
    println!("user2 is {:#?}", user2);
}
