use std::fs;

use crate::Token::token::*;
use std::str::Chars;
use std::iter::Peekable;

pub struct Parser {
    pub tree: ExprAST
}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        let contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens: Vec<Token> = Vec::new();
        let mut characters = contents.chars().peekable();

        Parser {
            tree: ExprAST {
                child: Self::recursive_descent(),
            }
        }
    }

    fn parse_identifier(identifier_name: String) -> VariableExprAST {
        VariableExprAST { 
            name: (identifier_name) 
        }
    }

    fn parse_unary(negate: bool, tok: Token, string_val: String) -> UnaryExprAST {
        match tok {
            Token::IDENTIFIER => {
                UnaryExprAST { 
                    negate: (negate), 
                    child: (Box::new(VariableExprAST {
                        name: string_val,
                    })
                )}
            },

            Token::NUMBER => {
                UnaryExprAST { 
                    negate: (negate), 
                    child: (Box::new(VariableExprAST {
                        name: string_val.parse().unwrap(),
                    })
                )}
            },

            _ => {
                panic!("Unary expression must have variable or number as a child.")
            }
        }
    }

    fn gettok(chars: &mut Peekable<Chars>) -> (Token, String) { //Returns and consumes the current token
        let mut tok_str = String::from("");

        while let Some(c) = chars.next() {

            if c == ' ' {
                loop {
                    if chars.peek() == Some(&' ') {
                        chars.next();
                        continue;
                    }

                    return (Token::WHITESPACE, tok_str);
                }
            }

            else if c == '\t' || c == '\n' {
                return (Token::MISC, tok_str);
            }

            else if c == '+' || c == '-' || c == '/' || c == '*' {
                return (Token::OPERATOR, String::from(c));
            }

            else if c == '=' {
                return (Token::EQUAL, String::from(c));
            }

            else if c == '(' {
                return (Token::OPENPARENT, String::from(c));
            }

            else if c == ')' {
                return (Token::CLOSEPARENT, String::from(c));
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

                return (Token::IDENTIFIER, tok_str);
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

                return (Token::NUMBER, tok_str);
            }

        }

        return (Token::EOF, String::from(""));
    }

    fn scantok(chars: &mut Peekable<Chars>) -> (Token, String) { //Returns the next token without consuming
        return Self::gettok(&mut chars.clone()); //TODO: Find a way to remove clone
    }

}