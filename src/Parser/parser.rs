use std::fs;
use crate::Token::token::*;

pub struct Parser {
    Root: ExprAST
}

impl Parser {
    pub fn new(filename: &String) -> Self {
        let contents = fs::read_to_string(filename)
                            .expect("FILE NAME NOT VALID");

        let mut tokens: Vec<Box<dyn Visit>> = Vec::new();

        let mut characters = contents.chars();
        let mut identifier_str: String = String::from("");

        let mut prev_identifier: VariableExprAST;

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
                        } else {
                            break;
                        }
                    }

                    match identifier_str.as_str() {
                        "def" => {
                            let def_obj: ExprAST = ExprAST {Tokens: Vec::new()}; //TODO: Create seperate struct for def
                            tokens.push(Box::new(def_obj));
                        }
                        "extern" => {
                            let extern_obj: ExprAST = ExprAST {Tokens: Vec::new()}; //TODO: Create seperate struct for extern
                            tokens.push(Box::new(extern_obj));
                        }
                        _ => {
                            !unimplemented!();
                        }
                    }

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
                    
                    let num: NumberExprAST = NumberExprAST { Value: (identifier_str.parse().unwrap()) };

                    tokens.push(Box::new(num)); //Storing num object in heap
                    identifier_str = String::from("");
                }

            } else { //EOF
                break;
            }
        }
        

        let parserExpr: ExprAST = ExprAST {
            Tokens: tokens,
        };

        Parser {
            Root: parserExpr
        }
    }


}