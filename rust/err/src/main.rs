use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){ 
                ErrorKind::NotFound => match File::create("hello.txt"){
                                        Ok(fc) => fc,
                                        Err(e) => panic!("Problem creating the file: {:?}", e),
                                    },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
