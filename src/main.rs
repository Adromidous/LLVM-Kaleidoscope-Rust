use std::fs;
use std::str::Chars;

pub mod Token;
pub mod Lexer;

use crate::{Token::token as tok, Lexer::lexer as lex};

struct Parser {
    Root: tok::ExprAST
}

impl Parser {

    fn new(filename: &String) -> Self {
        let contents = fs::read_to_string(filename)
                            .expect("FILE NAME NOT VALID");

        let mut identifier_str = String::from("");

        let mut par = contents.chars(); 

        let parserExpr = tok::ExprAST {
            Tokens: Vec::new()
        };

        Parser {
            Root: parserExpr
        }
    }

}

fn main() {
    let file_path: String = String::from("test_files/test2.txt");

    let mut lex1: lex::Lexer = lex::Lexer::new(&file_path);

    let mut parser1 = Parser::new(&file_path);
}
