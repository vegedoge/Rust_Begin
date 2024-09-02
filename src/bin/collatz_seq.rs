// n = n / 2 (n is even)
// n = 3n + 1 (n is odd)
// find the longest chain under 1M
fn main() {
    let mut counter: i64 = 1;
    let mut longest_num: i64= 1;
    let mut longest_length: i64= 1;
    for n in 1..= 1000000 {
        let mut x = n;
        counter = 1;
        while x > 1 {
            counter += 1;
            if x % 2 == 0 {
                x = x / 2;
            }
            else {
                x = 3 * x + 1;
            }
        }
        if counter > longest_length {
            longest_length = counter;
            longest_num = n;
        }
    }
    println!("{longest_num}")
}