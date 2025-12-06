use std::fs;
use std::str::Chars;

pub mod Token;
pub mod Lexer;
pub mod Parser;

use crate::{Token::token as tok, Lexer::lexer as lex, Parser::parser as par};

fn main() {
    let file_path: String = String::from("test_files/test2.txt");

    let parser1 = par::Parser::new(&file_path);
}
