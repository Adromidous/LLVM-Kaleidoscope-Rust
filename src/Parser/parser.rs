use crate::Token::token::{self as tok, BinaryExprAST, ExprAST, NumberExprAST, VariableExprAST};
use crate::Lexer::lexer as lex;

use std::fmt::Binary;
use std::str::Chars;

pub struct Parser {
    Root: tok::ExprAST
}

impl Parser {
    pub fn new(&self, lex: &mut lex::Lexer) -> Self {

        let mut ast: Vec<Box<dyn tok::Visit>> = Vec::new();

        let mut characters = lex.contents.chars();

        while (lex.tokens.len() > 0) {
            let tok = lex.tokens.pop_front().unwrap();

            match tok {
                tok::Token::EOF => break,

                tok::Token::Whitespace => continue,

                tok::Token::Identifier => {
                    assert_eq!(lex.tokens.pop_front().unwrap(), tok::Token::Number);

                    let val: NumberExprAST = NumberExprAST {
                        Value: self.gettok(&mut characters).parse().unwrap(),
                    };

                    let var: VariableExprAST = VariableExprAST { Name: (self.gettok(&mut characters))};

                    ast.push(Box::new(var));
                }

                tok::Token::Number => {
                    let val: NumberExprAST = NumberExprAST {
                        Value: self.gettok(&mut characters).parse().unwrap(),
                    };

                    ast.push(Box::new(val));
                }

                tok::Token::Operator => {
                    let op = self.gettok(&mut characters);

                    let lhs = ast.pop().unwrap();

                    let rhs = self.gettok(&mut characters);

                    let bin: BinaryExprAST;
                    
                    match self.token_lookahead(characters) {
                        tok::Token::Identifier => {
                            bin = BinaryExprAST {
                                Operator: op,
                                LHS: lhs,
                                RHS: Box::new(VariableExprAST { 
                                    Name: rhs,
                                }),
                            };
                        }

                        _ => {
                            bin = BinaryExprAST {
                                Operator: op,
                                LHS: lhs,
                                RHS: Box::new(NumberExprAST { 
                                    Value: rhs.parse().unwrap(),
                                }),
                            };
                        }
                    }

                    lex.tokens.pop_front().unwrap(); //Skip the next token since we have to look ahead.
                    ast.push(Box::new(bin));
                }

                _ => {
                    !unimplemented!();
                }
            }
        }

        Parser {
            Root: ExprAST {
                Children: ast,
            }
        }
    }

    //RETURNS THE VALUE OF TOKEN
    fn gettok(&self, char_iter: &mut Chars<'_>) -> String {
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

                    return identifier_str;
                } 

                else if valid_char.is_numeric() {   //NUMBER
                    identifier_str.push(valid_char);

                    while let Some(valid_char) = char_iter.next() {

                        if valid_char.is_numeric() {
                            identifier_str.push(valid_char);
                        } else {
                            break;
                        }
                    }
                    
                    return identifier_str;
                }

                else if valid_char == '+' || valid_char == '-' || valid_char == '*' || valid_char == '/'  { // OPERATOR
                    return String::from(valid_char);
                }
            }

        }

    }

    fn token_lookahead(&self, char_iter: &mut Chars<'_>) -> tok::Token {
        let mut lookahead = char_iter.peekable();
        let val = lookahead.peek().unwrap();

        if val.is_alphabetic() {
            tok::Token::Identifier
        } else {
            tok::Token::Number
        }
    }
}