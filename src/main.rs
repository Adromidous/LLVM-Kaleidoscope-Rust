pub mod Token;
pub mod Lexer;
pub mod Parser;

use crate::{Lexer::lexer as lex, Parser::parser as par};

fn main() {
    let file_path: String = String::from("test_files/test2.txt");

    let mut lex = lex::Lexer::new(&file_path);
    let parse1 = par::Parser::new(&mut lex);
    //parse1.explore_tree();
}
