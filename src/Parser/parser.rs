use crate::Token::token::{self as tok, BinaryExprAST, ExprAST, NumberExprAST, VariableExprAST};
use crate::Lexer::lexer as lex;

use std::fmt::Binary;
use std::str::Chars;

pub struct Parser {
    Root: Box<dyn tok::Visit>,
}

impl Parser {

    pub fn new(&self, lex: &mut lex::Lexer) {
        let mut characters = lex.contents.chars();

        Parser {
            Root: Self::recursive_descent(characters),
        };
    }

    fn recursive_descent(char_iter: &mut Chars<'_>) -> Box<dyn tok::Visit> { 
        
        let (curr_tok, curr_str) = Self::gettok(char_iter);

        match curr_tok {
            tok::Token::Identifier => {
                return Box::new(VariableExprAST { Name: curr_str});
            },

            tok::Token::Number => {
                return Box::new(NumberExprAST { Value: curr_str.parse().unwrap()});
            },

            tok::Token::Operator => {
                return Box::new(BinaryExprAST { 
                    Operator: curr_str.parse().unwrap(),
                    LHS: Self::recursive_descent(char_iter),
                    RHS: Self::recursive_descent(char_iter),
                });
            },

            _ => {
                !todo!()
            }
        }
    }

    //RETURNS THE VALUE OF TOKEN
    fn gettok(char_iter: &mut Chars<'_>) -> (tok::Token, String){
        let mut identifier_str: String = String::from("");

        let character = char_iter.next();

        loop {
            if let Some(valid_char) = character {

                if valid_char == ' ' { //WHITESPACES
                    continue;
                }

                else if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
                    identifier_str.push(valid_char);
                    
                    while let Some(valid_char) = char_iter.next() {
                        
                        if valid_char.is_alphabetic() || valid_char.is_numeric() {
                            identifier_str.push(valid_char);    
                        } else {
                            break;
                        }

                    }

                    return (tok::Token::Identifier, identifier_str);
                } 

                else if valid_char.is_numeric() { //NUMBER
                    identifier_str.push(valid_char);

                    while let Some(valid_char) = char_iter.next() {

                        if valid_char.is_numeric() {
                            identifier_str.push(valid_char);
                        } else {
                            break;
                        }
                    }

                    return (tok::Token::Number, identifier_str);
                }

                else if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/'  { // OPERATOR
                    return (tok::Token::Operator, String::from(valid_char));
                }
            }
        }
    }
}