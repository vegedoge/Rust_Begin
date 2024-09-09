pub fn fibonacci(n: u64) -> u64 {
    let mut F_last = 1;
    let mut helper = 0;
    let mut F = 0;
    for _ in 0..n {
        helper = F;
        F = F + F_last;
        F_last = helper;
    }
    F
}

fn main() {
    let N = fibonacci(4);
    println!("{N}");
}