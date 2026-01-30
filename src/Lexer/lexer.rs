use std::fs;

use crate::Token::token::*;
use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer {
    pub contents: String,
    pub tokens: Vec<Token>
}

impl Lexer {
    pub fn new(filename: &str) -> Lexer {
        let contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens: Vec<Token> = Vec::new();
        let mut characters = contents.chars().peekable();

        loop {
            let tok = Self::gettok(&mut characters);
            
            if tok == Token::EOF {
                tokens.push(tok);
                break;
            }

            else {
                tokens.push(tok);
            }
        }
    
        Lexer {
            contents: contents,
            tokens: tokens 
        }

    }

    fn gettok(chars: &mut Peekable<Chars>) -> Token { //Returns next token
        let mut tok_str = String::from("");

        while let Some(c) = chars.next() {

            if c == ' ' {
                loop {
                    if chars.peek() == Some(&' ') {
                        chars.next();
                        continue;
                    }

                    return Token::WHITESPACE;
                }
            }

            else if c == '\t' || c == '\n' {
                return Token::MISC;
            }

            else if c == '+' || c == '-' || c == '/' || c == '*' {
                return Token::OPERATOR;
            }

            else if c == '=' {
                return Token::EQUAL;
            }

            else if c == '(' {
                return Token::OPENPARENT;
            }

            else if c == ')' {
                return Token::CLOSEPARENT;
            }

            else if c.is_alphabetic() {
                tok_str.push(c);

                while let Some(&c_next) = chars.peek() {
                    if c_next.is_alphanumeric() {
                        tok_str.push(c_next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                return Token::IDENTIFIER;
            }

            else if c.is_numeric() {
                tok_str.push(c);

                while let Some(&c_next) = chars.peek() {
                    if c_next.is_numeric() {
                        tok_str.push(c_next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                return Token::NUMBER;
            }

        }

        return Token::EOF;
    }

    pub fn print_tokens(&self){
        for tok in self.tokens.iter() {
            println!("TOKEN: {:#?}", tok);
        }
    }

}