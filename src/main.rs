use std::env;
use std::fs;
use std::io;

enum Token {
    EOF,

    // Commands
    Def,
    Extern,

    // Primary
    Identifier,
    Number
}

fn main() {
    let file_path = String::from("test_files/test1.txt");
    let contents = fs::read_to_string(file_path)
                    .expect("FILE NAME NOT VALID");

    for i in contents.chars() {
        println!("{}", i);
    }
     
}
