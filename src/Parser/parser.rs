use std::fs;

use crate::Token::token::*;
use std::str::Chars;
use std::iter::Peekable;

/*
GRAMMAR RULES

    Expression  => Term

    Term        => Factor ( ( "-" | "+" ) Factor )*

    Factor      => Unary ( ( "*" | "/" ) Unary )*

    Unary       => ("!" | "-") Unary | Primary
    
    Primary     => NUMBER | STRING | "true" | "false" | "null" | "(" Expression ")"

*/


pub enum ExprAST {
    NumberExprAST{value: usize},
    VariableExprAST{identifier: String},
    LiteralExprAST{literal: Box<ExprAST>},
    UnaryExprAST{value: Box<ExprAST>},
    BinaryExprAST{op: String, lhs: Box<ExprAST>, rhs: Box<ExprAST>},
    Error,
    Null,
    EOFExprAST
}

pub enum Bool {
    True,
    False
}

pub struct Parser {
    pub tree: ExprAST,
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
                println!("(");
                lhs.print();
                println!("{}", op);
                rhs.print();
                println!(")");
            },

            ExprAST::LiteralExprAST { literal } => {
                literal.print();
            },

            ExprAST::Null => {
                println!("NULL");
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

        let mut characters = contents.chars().peekable();

        Parser {
            tree: Self::recursive_descent(&mut characters),
        }
    }

    fn recursive_descent(chars: &mut Peekable<Chars>) -> ExprAST {
        return Self::parse_expression(chars);
    }

    fn parse_expression(chars: &mut Peekable<Chars>) -> ExprAST {
        return Self::parse_term(chars);
    }

    fn parse_term(chars: &mut Peekable<Chars>) -> ExprAST {
        let lhs: ExprAST = Self::parse_factor(chars);

        let (next_tok, next_str) = Self::scantok(chars);

        match next_tok {
            
            Token::OPERATOR => {

                if next_str == "+" || next_str == "-" {
                    let (op_tok, op_str) = Self::gettok(chars);
                    let rhs: ExprAST = Self::parse_term(chars);

                    return ExprAST::BinaryExprAST { op: (op_str), lhs: (Box::new(lhs)), rhs: (Box::new(rhs)) }
                } else {
                    return lhs;
                }
            }

            _ => {
                return lhs;
            }

        }

    }

    fn parse_factor(chars: &mut Peekable<Chars>) -> ExprAST {
        let lhs: ExprAST = Self::parse_unary(chars);

        let (next_tok, next_str) = Self::scantok(chars);

        match next_tok {
            
            Token::OPERATOR => {

                if next_str == "*" || next_str == "/" {
                    let (op_tok, op_str) = Self::gettok(chars);
                    let rhs: ExprAST = Self::parse_unary(chars);

                    return ExprAST::BinaryExprAST { op: (op_str), lhs: (Box::new(lhs)), rhs: (Box::new(rhs)) }
                } else {
                    return lhs;
                }
            }

            _ => {
                return lhs;
            }

        }

    }

    fn parse_unary(chars: &mut Peekable<Chars>) -> ExprAST {
        let (tok, str_val) = Self::scantok(chars);

        match tok {

            Token::NEGATE => {
                let rhs: ExprAST = Self::parse_unary(chars);

                return ExprAST::UnaryExprAST { value: (Box::new(rhs)) }
            },

            _ => {
                return Self::parse_primary(chars);
            }
        }
    }

    fn parse_primary(chars: &mut Peekable<Chars>) -> ExprAST {
        let (tok, str_val) = Self::gettok(chars);

        match tok {
            Token::IDENTIFIER => {
                return Self::parse_variable(str_val);
            },

            Token::NUMBER => {
                return Self::parse_number(str_val.parse().unwrap());
            },

            Token::BOOLEAN => {
                if str_val == "false" {
                    return Self::parse_number(0);
                } else {
                    return Self::parse_number(1);
                }
            },

            Token::NULL => {
                return ExprAST::Null;
            },

            Token::OPENPARENT => {
                return Self::parse_expression(chars);
            },

            _ => {
                return ExprAST::Error;
            }
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

            else if c == '+' || c == '-' || c == '/' || c == '*' || c == '=' {
                return (Token::OPERATOR, String::from(c));
            }

            else if c == '!' || c == '-' {
                return (Token::NEGATE, String::from(c));
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

                if tok_str == "true" || tok_str == "false" {
                    return (Token::BOOLEAN, tok_str);
                } else if tok_str == "null" {
                    return (Token::NULL, tok_str);
                } else {
                    return (Token::IDENTIFIER, tok_str);
                }
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
        return Self::gettok(&mut chars.clone());
    }

}
