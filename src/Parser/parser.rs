use std::fs;

use crate::Token::token::*;
use std::str::Chars;
use std::iter::Peekable;

pub struct Parser {
    pub tree: ExprAST
}

pub enum ExprAST {
    NumberExprAST(usize),
    VariableExprAST(String),
    UnaryExprAST{negate: bool, value: Box<ExprAST>},
    BinaryExprAST{operator: String, lhs: Box<ExprAST>, rhs: Box<ExprAST>},
    ParentExprAST(Vec<ExprAST>),
    Error,
    EOFExprAST{}
}

trait Visit {
    fn print(&self);
}

impl Visit for ExprAST {

    fn print(&self) {
       
        match self {
        
            ExprAST::NumberExprAST => {
                println!("{}", self);
            },

            ExprAST::VariableExprAST => {
                println!("", self);
            },

            ExprAST::BinaryExprAST => {

            }

            _ => {
                println!("");
            }

        }

    }

}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        let contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens: Vec<Token> = Vec::new();
        let mut characters = contents.chars().peekable();

        Parser {
            tree: ExprAST {
                child: Self::recursive_descent(&mut characters),
            }
        }
    }

    fn recursive_descent(chars: &mut Peekable<Chars>) -> ExprAST {

        loop {
            let (tok, val) = Self::gettok(chars); 

            if tok == Token::WHITESPACE || tok == Token::MISC {
                continue;
            } else if tok != Token::EOF {  

                match tok {

                    Token::IDENTIFIER => {
                        
                        let (tok_scan, val_scan) = Self::scantok(chars);

                        match tok_scan => {

                            Token::OPERATOR | Token::EQUAL => {
                                Self::gettok(chars); //Consume token
                                
                                let lhs: ExprAST::VariableExprAST(val); 

                                Self::parse_binary_expr(val_scan, lhs, Self::recursive_descent(chars));   
                            }

                            _ => {
                                Self::parse_variable(val)
                            }

                        }
                    },

                    Token::NUMBER => {

                        let (tok_scan, val_scan) = Self::scantok(chars);

                        match tok_scan => {

                            Token::OPERATOR | Token::EQUAL => {
                                Self::gettok(chars); //Consume token
                                
                                let lhs: ExprAST::NumberExprAST(val.parse().unwrap()); 

                                Self::parse_binary_expr(val_scan, lhs, Self::recursive_descent(chars));   
                            }

                            _ => {
                                Self::parse_variable(val)
                            }

                        }

                    },


                    Token::OPERATOR | Token::EQUAL | Token::CLOSEPARENT => {
                        ExprAST::Error
                    },

                    _ => {
                        ExprAST::EOFExprAST { }
                    }

                }

            }

        ExprAST::EOFExprAST {  }
        }
        
    }

    fn parse_number(value: usize) -> ExprAST::NumberExprAST {
        ExprAST::NumberExprAST(value)
    }

    fn parse_variable(identifier: String) -> ExprAST::VariableExprAST {
        ExprAST::VariableExprAST(identifier)
    }

    fn print_tree(&self) {
        self.print();
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
