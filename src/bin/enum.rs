enmu Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main(){
    let m1 = Message::Quit;
    let m2 = Message::Move{x: 12, y: 18};
    let m3 = Message::ChangeColor(255, 0, 255);

}