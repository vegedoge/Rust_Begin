
// practice for panic and result<T, E>
use std::{fs::File, io::ErrorKind};

fn main(){
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,

        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error)
        },
        
    };
}