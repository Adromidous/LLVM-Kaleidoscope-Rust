use std::fs;

use crate::Token::token::*;
use std::str::Chars;
use std::iter::Peekable;

pub enum ExprAST {
    NumberExprAST{value: usize},
    VariableExprAST{identifier: String}, 
    UnaryExprAST{value: Box<ExprAST>},
    BinaryExprAST{op: String, lhs: Box<ExprAST>, rhs: Box<ExprAST>},
    ParentExprAST{children: Vec<ExprAST>},
    Error,
    EOFExprAST
}

pub struct Parser {
    pub tree: ExprAST
}

trait Visit {
    fn print(&self);
}

impl Visit for ExprAST {

    fn print(&self) {
       
        match self {
        
            ExprAST::NumberExprAST{ value } => {
                println!("{}", value);
            },

            ExprAST::VariableExprAST{ identifier } => {
                println!("{}", identifier);
            },

            ExprAST::UnaryExprAST { value } => {
                print!("-");
                value.print();
            },

            ExprAST::BinaryExprAST { op, lhs, rhs } => {
                lhs.print();
                println!("{}", op);
                rhs.print();
            },

            ExprAST::ParentExprAST { children } => {
                println!("(");

                for node in children {
                    node.print();
                }

                println!(")");
            },

            ExprAST::Error => {
                println!("ERROR!!!");
            },

            ExprAST::EOFExprAST => {
                println!("EOF");
            },

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
            tree: Self::recursive_descent(&mut characters),
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

                        match tok_scan {

                            Token::OPERATOR | Token::EQUAL => {
                                Self::gettok(chars); //Consume token
                                
                                let lhs = ExprAST::VariableExprAST{
                                    identifier: val
                                }; 

                                return Self::parse_binary_expr(val_scan, lhs, Self::recursive_descent(chars));   
                            }

                            _ => {
                                return Self::parse_variable(val);
                            }

                        }
                    },

                    Token::NUMBER => {

                        let (tok_scan, val_scan) = Self::scantok(chars);

                        match tok_scan {

                            Token::OPERATOR => {
                                Self::gettok(chars); //Consume token
                                
                                let lhs = ExprAST::NumberExprAST{
                                    value: val.parse().unwrap()
                                }; 

                                return Self::parse_binary_expr(val_scan, lhs, Self::recursive_descent(chars));   
                            }

                            _ => {
                                return Self::parse_number(val.parse().unwrap());
                            }

                        }

                    },

                    Token::OPERATOR => {
                        
                        match val.as_str() {
                            
                            "-" => { //Negation to create UnaryExprAST object
                                let nextExprAST = Self::recursive_descent(chars);             
                                
                                return ExprAST::UnaryExprAST{
                                    value: Box::new(nextExprAST)
                                };
                            }

                            _ => {
                                return ExprAST::Error;
                            }

                        }

                    }


                    Token::OPENPARENT => {
                        let mut children: Vec<ExprAST> = Vec::new(); 
                        
                        loop {
                            let (tok_scan, val_scan) = Self::scantok(chars);
                            
                            match tok_scan {

                                Token::CLOSEPARENT => { 
                                    return ExprAST::ParentExprAST {
                                        children: children
                                    };
                                },


                                Token::WHITESPACE | Token::MISC => {
                                    let _ = Self::gettok(chars);
                                    continue;
                                },

                                Token::EOF | Token::OPERATOR | Token::EQUAL => {
                                    return ExprAST::Error;
                                },

                                _ => {
                                    children.push(Self::recursive_descent(chars));
                                }
                            }
                        }
                    }


                    Token::EQUAL | Token::CLOSEPARENT => {
                        return ExprAST::Error;
                    },

                    _ => {
                        return ExprAST::Error;
                    }

                }

            }

            return ExprAST::EOFExprAST{};
        }
    }

    fn parse_number(value: usize) -> ExprAST {
        ExprAST::NumberExprAST{
            value: value
        }
    }

    fn parse_variable(identifier: String) -> ExprAST {
        ExprAST::VariableExprAST{
            identifier: identifier
        }
    }

    fn parse_binary_expr(op: String, lhs: ExprAST, rhs: ExprAST) -> ExprAST {
        ExprAST::BinaryExprAST {
            op: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }

    pub fn print_tree(&self) {
        self.tree.print();
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
