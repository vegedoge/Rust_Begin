// This is the difference between statement and expression
fn main(){
    let y = {
        let x = 3;
        x + 3
    };
    println!("The value of y is {}", y);
}