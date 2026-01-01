use crate::Token::token::{self as tok, BinaryExprAST, ExprAST, NumberExprAST, VariableExprAST};
use crate::Lexer::lexer as lex;

use std::str::Chars;

pub struct Parser {
    root: Box<dyn tok::Visit>,
}

impl Parser {

    pub fn new(lex: &mut lex::Lexer) -> Parser{
        let mut characters = lex.contents.chars();

        Parser {
            root: Self::recursive_descent(&mut characters),
        }
    }

    fn recursive_descent(char_iter: &mut Chars<'_>) -> Box<dyn tok::Visit> { 
        let (curr_tok, curr_str) = Self::gettok(char_iter);

        match curr_tok {
            tok::Token::EOF => {
                return Box::new(
                    ExprAST {
                        child: Vec::new(), //FIXME: Need to find alternate solution to EOF
                    }
                );
            },

            tok::Token::Identifier => {
                if Self::lookahead_tok(char_iter) == tok::Token::Operator {
                    return Box::new(BinaryExprAST {
                        lhs: Box::new(VariableExprAST {name: curr_str}),
                        operator: String::from(Self::gettok(char_iter).1),
                        rhs: Self::recursive_descent(char_iter)
                    })
                } else {
                    return Box::new(VariableExprAST { name: curr_str});
                }
            },

            tok::Token::Number => {
                return Box::new(NumberExprAST { value: curr_str.parse().unwrap()});
            },

            tok::Token::Operator => {
                return Box::new(BinaryExprAST { 
                    operator: curr_str.parse().unwrap(),
                    lhs: Self::recursive_descent(char_iter),
                    rhs: Self::recursive_descent(char_iter),
                });
            },
        }
    }

    //RETURNS THE VALUE OF TOKEN
    fn gettok(char_iter: &mut Chars<'_>) -> (tok::Token, String){
        let mut identifier_str: String = String::from("");
        loop {
            if let Some(valid_char) = char_iter.next() {
                
                if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
                    identifier_str.push(valid_char);
                    while let Some(valid_char) = char_iter.next() {
                        if valid_char.is_alphanumeric() {
                            identifier_str.push(valid_char);                    
                        } else {
                            break;
                        }
                    }

                    return (tok::Token::Identifier, identifier_str)
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

                if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/' || valid_char == '=' { //OPERATOR
                    return (tok::Token::Operator, String::from(valid_char));
                }

                else {
                    continue; //WHITESPACE
                }
            } else { //EOF
                return (tok::Token::EOF, String::from(""));
            }
        }
    }

    fn lookahead_tok(char_iter: &mut Chars<'_>) -> tok::Token {

        let mut lookahead_itr = char_iter.peekable();

        let lookahead_char = lookahead_itr.peek();

        if lookahead_char == Some(&'+') || lookahead_char == Some(&'-') || lookahead_char == Some(&'*') || lookahead_char == Some(&'/') || lookahead_char == Some(&'=') {
            return tok::Token::Operator;
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