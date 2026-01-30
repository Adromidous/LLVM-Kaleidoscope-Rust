use std::fs;

use crate::Token::token::*;
use std::str::Chars;

pub struct Lexer {
    pub contents: String,
    pub tokens: Vec<Token>
}

impl Lexer {
    pub fn new(filename: &str) -> Lexer {
        let contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens: Vec<Token> = Vec::new();

        let mut characters = contents.chars();
        let mut peekable_chars = characters.peekable();
        let mut identifier_str: String = String::from("");

        loop {
            let character = characters.next();

            if let Some(valid_char) = character {

                if valid_char == ' ' { //WHITESPACES
                    continue;
                }

                else if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
                    identifier_str.push(valid_char);
                    
                    while let Some(valid_char) = characters.next() {
                        
                        if valid_char.is_alphabetic() || valid_char.is_numeric() {
                            identifier_str.push(valid_char);    
                        }

                        else {
                            break;
                        }

                    }

                    tokens.push(Token::IDENTIFIER);
                    identifier_str = String::from("");

                } 

                else if valid_char.is_numeric() {   //NUMBER
                    identifier_str.push(valid_char);

                    while let Some(valid_char) = characters.next() {

                        if valid_char.is_numeric() {
                            identifier_str.push(valid_char);
                        }

                        else {
                            break;
                        }
                    }
                    
                    tokens.push(Token::NUMBER);
                    identifier_str = String::from("");
                }

                else if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/' || valid_char == '=' { // OPERATOR
                    tokens.push(Token::OPERATOR);
                }

            } else { //EOF
                tokens.push(Token::EOF);
                break;
            }

        }
    
        Lexer {
            contents: contents,
            tokens: tokens 
        }

    }

    fn gettok(chars: &mut Chars<'_>) -> Token { //Returns next token
        let mut tok_str = String::from("");

        while let Some(c) = chars.next() {

            if c == ' ' {
                loop {
                    if chars.next() == Some(' ') {
                        continue;
                    }

                    return Token::WHITESPACE;
                }
            }

            else if c == '\t' || c == '\n' {
                return Token::MISC;
            }

            else if c == '\0' {
                return Token::EOF;
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

                while let Some(c_next) = chars.next() {
                    if c_next.is_alphanumeric() {
                        tok_str.push(c_next);
                    } else {
                        return Token::IDENTIFIER;
                    }
                }
            }

            else if c.is_numeric() {
                tok_str.push(c);

                while let Some(c_next) = chars.next() {
                    if c_next.is_numeric() {
                        tok_str.push(c_next);
                    } else {
                        return Token::NUMBER;
                    }
                }
            }

        }

        return Token::MISC;
    }

    pub fn print_tokens(&self){
        for tok in self.tokens.iter() {
            println!("TOKEN: {:#?}", tok);
        }
    }

}