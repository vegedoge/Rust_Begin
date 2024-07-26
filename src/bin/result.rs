
// practice for panic and result<T, E>
use std::fs::File;

fn main(){
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem occured while opening {:?}", error)
        },
    };
}