// test for match func
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main(){
    let coin1 = Coin::Dime;
    let coin2 = Coin::Quarter(UsState::Alaska);
    let value1 = value_in_cents(coin1);
    let value2 = value_in_cents(coin2);
    println!("value1 is {}, value2 is {}", value1, value2);

}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        },
    }
}
