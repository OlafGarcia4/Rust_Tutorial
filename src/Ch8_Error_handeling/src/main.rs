//use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let v = vec![1, 2, 3];
    let greeting_file_result = File::open("hello.txt");
    //v[99];
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error)=> panic!("Problem opening file {:?}", error)
    } ;
    println!("No errors, yay!");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}