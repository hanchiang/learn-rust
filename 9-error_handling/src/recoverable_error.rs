use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn main() {

}

pub fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => {
            println!("Found file hello.txt");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Created file hello.txt");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
}

// This function has the same behaviour as open_file(), but it does not have match blocks
// and is cleaner to read
pub fn open_file_better() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file {:?}", error)
        }
    });
    println!("Found file hello.txt");
}

pub fn propagate_error() -> Result<String, io::Error> {
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

pub fn propagate_error_better() -> Result<String, io::Error> {
    // This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn propagate_error_best() -> Result<String, io::Error> {
    // This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}