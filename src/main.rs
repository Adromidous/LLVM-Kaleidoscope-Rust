pub mod Token;
pub mod Lexer;
pub mod Parser;

use std::{fs::File, io::Write};
use crate::{Lexer::lexer as lex, Parser::parser as par};

fn write_asm_header(asm_file: &mut File) {
    asm_file.write_all(b".global _main\n");
    asm_file.write_all(b".text\n");
    asm_file.write_all(b"_main:\n");
    asm_file.write_all(b"ret\n");
}

fn main() {
    let mut asm_file = File::create("prog.s");
    write_asm_header(&mut asm_file.unwrap());

    let file_path: String = String::from("test_files/test5.txt");
    
    let mut lex = lex::Lexer::new(&file_path);
    let parse1 = par::Parser::new(&mut lex);
    parse1.explore_tree();
}
