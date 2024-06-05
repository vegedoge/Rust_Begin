// entrance function fn
fn main(){
    let a = 10;
    // type: i32, value: 20
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);

}

fn add(i: i32, j: i32) -> i32 {
    // return can be skipped here
    i + j
}