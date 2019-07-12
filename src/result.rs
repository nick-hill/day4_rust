use std::fs::File;
use std::io::ErrorKind;

pub fn res() {

    let f = File::open("/home/knoldus/Desktop/day4_rust/src/hello.txt");

    //using expect
    f.expect("FAILED TO OPEN");

    //using unwrap
    f.unwrap();

    //manually using ErrorKind
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("Problem creating the file: {:?}", _e),
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    println!("File created");
}