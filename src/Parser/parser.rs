use crate::Token::token::{self as tok, BinaryExprAST, ExprAST, NumberExprAST, VariableExprAST};
use crate::Lexer::lexer as lex;

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

                    let var: VariableExprAST = VariableExprAST { Name: (self.gettok(&mut characters)), Value: (val) };

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

                    let lhs = *(ast.pop().unwrap());

                    assert_eq!(lex.tokens.pop_front().unwrap(), tok::Token::Number);
                    let rhs = self.gettok(&mut characters);

                    let bin = BinaryExprAST {
                        Operator: op,
                        LHS: NumberExprAST { Value: (lhs.Value) },
                        RHS: NumberExprAST { Value: (rhs.parse().unwrap()) },
                    };

                    ast.push(Box::new(bin));
                }

                _ => {
                    !unimplemented!();
                }
            }
        }

        Parser {
            Root: ExprAST {
                Tokens: ast,
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
}