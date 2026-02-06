use std::ops::Deref;
use std::{fs::File, io::Write};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Commands
    // Def,                //['d', 'e', 'f']
    // Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    IDENTIFIER,         //[a-zA-Z][a-zA-Z0-9]*
    NUMBER,             //[0-9]+
    OPERATOR,           //['+', '-', '*', '/']
    EQUAL,              //['=']

    OPENPARENT,
    CLOSEPARENT,
    WHITESPACE,
    MISC,               //['\t', '\n']
    EOF                 //['\0']
}
