// 2520 is the smallest number that can be divided by each 
// of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly 
// divisible by all of the numbers from 1 to 20? 

fn main() {
    'outer: for i in (2..).step_by(2) {
        for j in 1..=20 {
            if i % j != 0 {
                continue 'outer
            }
        }
        println!("{i}");
        return
    }
}

