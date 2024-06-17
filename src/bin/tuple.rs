// tuple
fn main(){
    let tuple: (u64, i32, f32) = (1000, 52, 3.3);
    let (_x, y, _z) = tuple;
    println!("The value of y is {}", y);

    let _one_k = tuple.0;

}