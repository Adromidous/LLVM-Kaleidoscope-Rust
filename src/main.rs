pub mod Token;
pub mod Lexer;
pub mod Parser;

use std::{fs::File, io::Write};
use crate::{Lexer::lexer as lex, Parser::parser as par};

fn main() {
    let file_path: String = String::from("test_files/arithmetic_test2.txt");
    
    //let mut lex = lex::Lexer::new(&file_path);
    //lex.print_tokens();

    let parse1 = par::Parser::new(&file_path);
    parse1.print_tree();
}
