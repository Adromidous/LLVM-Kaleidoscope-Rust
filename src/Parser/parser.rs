use crate::Token::token::{self as tok, BinaryExprAST, EOFExprAST, EqualExprAST, ExprAST, NumberExprAST, ParenthesisExprAST, VariableExprAST};
use crate::Lexer::lexer as lex;

use std::{str::Chars, iter::Peekable};

pub struct Parser {
    root: Box<dyn tok::Visit>,
}

impl Parser {

    pub fn new(lex: &mut lex::Lexer) -> Parser{
        let mut characters = lex.contents.chars().peekable();

        Parser {
            root: Self::recursive_descent(&mut characters),
        }
    }

    fn recursive_descent(char_iter: &mut Peekable<Chars>) -> Box<dyn tok::Visit> { 
        let (curr_tok, curr_str) = Self::gettok(char_iter);

        match curr_tok {

            tok::Token::Identifier => {
                if Self::lookahead_tok(char_iter) == tok::Token::Operator {
                    return Box::new(BinaryExprAST {
                        lhs: Box::new(VariableExprAST {name: curr_str}),
                        operator: Self::gettok(char_iter).1,
                        rhs: Self::recursive_descent(char_iter)
                    })
                } else if Self::lookahead_tok(char_iter) == tok::Token::Equal {
                    Self::gettok(char_iter);
                    return Box::new(EqualExprAST {
                        lhs: VariableExprAST {name: curr_str},
                        rhs: Self::recursive_descent(char_iter)
                    })
                } else {      
                    return Box::new(VariableExprAST { name: curr_str});
                }
            },

            tok::Token::OpenParen => {

                let ret_tok = Box::new(ParenthesisExprAST { 
                    child: Self::recursive_descent(char_iter),
                });

                assert_eq!(Self::gettok(char_iter).0, tok::Token::CloseParen);

                return ret_tok;
            }

            tok::Token::Number => {
                let value: usize = curr_str.parse().unwrap();

                if Self::lookahead_tok(char_iter) == tok::Token::Operator {
                    let op = Self::gettok(char_iter);

                    if Self::lookahead_tok(char_iter) == tok::Token::Number {
                        let rhs_value: usize = Self::gettok(char_iter).1.parse().unwrap();
                        match op.1.as_str() {
                            "+" => {
                                return Box::new(NumberExprAST {
                                    value: value + rhs_value,
                                })
                            },

                            "-" => {
                                return Box::new(NumberExprAST {
                                    value: value - rhs_value,
                                })
                            },

                            "/" => {
                                return Box::new(NumberExprAST {
                                    value: value / rhs_value,
                                })
                            },

                            "*" => {
                                return Box::new(NumberExprAST {
                                    value: value * rhs_value,
                                })
                            },

                            _ => {
                                println!("Error!");
                            }
                        }
                    }
                    return Box::new(BinaryExprAST {
                        lhs: Box::new(NumberExprAST { value: value}),
                        operator: op.1,
                        rhs: Self::recursive_descent(char_iter)
                    })
                } else {
                    return Box::new(NumberExprAST { value: value});
                }
            },

            tok::Token::EOF => {
                return Box::new(EOFExprAST{})
            },

            _ => { // FIXME: COVER CASES FOR OPERATOR
                return Box::new(
                    ExprAST {
                        child: Vec::new(), //FIXME: NEED TO FIND ALTERNATE SOLUTION FOR EOF
                    }
                );
            },

        }
    }

    fn gettok(char_iter: &mut Peekable<Chars>) -> (tok::Token, String){
        let mut identifier_str: String = String::from("");

        loop {

            if let Some(valid_char) = char_iter.next() {

                if valid_char.is_alphabetic() { //IDENTIFIER
                    identifier_str.push(valid_char);

                    while let Some(&next_char) = char_iter.peek() {
                        if next_char.is_alphanumeric() {
                            identifier_str.push(next_char);
                            char_iter.next();               
                        } else {
                            break;
                        }
                    }

                    if identifier_str == "def" {
                        return (tok::Token::Def, identifier_str);
                    } else if identifier_str == "extern" {
                        return (tok::Token::Extern, identifier_str);
                    } else {
                        return (tok::Token::Identifier, identifier_str);
                    }
                }

                else if valid_char.is_numeric() { //NUMBER
                    identifier_str.push(valid_char);

                    while let Some(&valid_char) = char_iter.peek() {
                        if valid_char.is_numeric() {
                            identifier_str.push(valid_char);
                            char_iter.next();
                        } else {
                            break;
                        }
                    }

                    return (tok::Token::Number, identifier_str);
                }

                else if valid_char == '=' {
                    return (tok::Token::Equal, String::from(valid_char));
                }

                else if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/' { //OPERATOR
                    return (tok::Token::Operator, String::from(valid_char));
                }

                else if valid_char == '(' { //OPEN PARENTHESIS
                    return (tok::Token::OpenParen, String::from("("));
                }

                else if valid_char == ')' { //CLOSE PARENTHESIS - Need this for error checking
                    return (tok::Token::CloseParen, String::from(")"));
                }

                else if valid_char == ',' {
                    return (tok::Token::Comma, String::from(","));
                }

                else { //WHITESPACES
                    continue;
                }

            } else { //EOF
                return (tok::Token::EOF, String::from(""));
            }
        }
    }

    fn lookahead_tok(char_iter: &mut Peekable<Chars>) -> tok::Token {

        while let Some(&peek_char) = char_iter.peek() {
            if peek_char == ' ' {
                char_iter.next();
            } else {
                break;
            }
        }

        let lookahead_char = char_iter.peek();

        if lookahead_char == Some(&'+') || lookahead_char == Some(&'-') || lookahead_char == Some(&'*') || lookahead_char == Some(&'/') {
            return tok::Token::Operator;
        } else if lookahead_char == Some(&'=') {
            return tok::Token::Equal;
        } else if lookahead_char.unwrap().is_numeric() {
            return tok::Token::Number;
        } else if lookahead_char.unwrap().is_alphabetic() {
            return tok::Token::Identifier;
        } else {
            return tok::Token::EOF;
        }
    }

    pub fn explore_tree(&self) {
        self.root.print();
    }
}