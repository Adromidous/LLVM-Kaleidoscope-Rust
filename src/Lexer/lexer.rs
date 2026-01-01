use std::{fs, collections::VecDeque};

use crate::Token::token::*;

pub struct Lexer {
    pub contents: String,
    pub tokens: VecDeque<Token>
}

impl Lexer {
    pub fn new(filename: &str) -> Lexer {
        let contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens: VecDeque<Token> = VecDeque::new();

        let mut characters = contents.chars();
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

                    // match identifier_str.as_str() {
                    //     "def" => tokens.push_back(Token::Def),
                    //     "extern" => tokens.push_back(Token::Extern),
                    //     _ => tokens.push_back(Token::Identifier),
                    // }

                    tokens.push_back(Token::Identifier);
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
                    
                    tokens.push_back(Token::Number);
                    identifier_str = String::from("");
                }

                else if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/' || valid_char == '=' { // OPERATOR
                    tokens.push_back(Token::Operator);
                }

            } else { //EOF
                tokens.push_back(Token::EOF);
                break;
            }

        }
    
        Lexer {
            contents: contents,
            tokens: tokens 
        }

    }

    pub fn print_tokens(&self){
        for tok in self.tokens.iter() {
            println!("TOKEN: {:#?}", tok);
        }
    }

}