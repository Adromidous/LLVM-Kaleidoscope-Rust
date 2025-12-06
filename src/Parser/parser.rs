use crate::Token::token::{self as tok, ExprAST};
use crate::Lexer::lexer as lex;

pub struct Parser {
    Root: tok::ExprAST
}

impl Parser {
    pub fn new(lex: &mut lex::Lexer) -> Self {

        let mut ast: Vec<Box<dyn tok::Visit>> = Vec::new();

        let mut characters = lex.contents.chars();
        let mut identifier_str: String = String::from("");

        while (lex.tokens.len() > 0) {
            let tok = lex.tokens.pop_front().unwrap();

            match tok {
                tok::Token::EOF => break,

                tok::Token::Identifier => {
                    
                }
            }
        }

        Parser {
            Root: ExprAST {
                Tokens: ast,
            }
        }
    //     loop {
    //         let character = characters.next();

    //         if let Some(valid_char) = character {

    //             if valid_char == ' ' { //WHITESPACES
    //                 continue;
    //             }

    //             else if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
    //                 identifier_str.push(valid_char);
                    
    //                 while let Some(valid_char) = characters.next() {
    //                     if valid_char.is_alphabetic() || valid_char.is_numeric() {
    //                         identifier_str.push(valid_char);    
    //                     } else {
    //                         break;
    //                     }
    //                 }

    //                 match identifier_str.as_str() {
    //                     "def" => {
    //                         let def_obj: tok::ExprAST = tok::ExprAST {Tokens: Vec::new()}; //TODO: Create seperate struct for def
    //                         ast.push(Box::new(def_obj));
    //                     }
    //                     "extern" => {
    //                         let extern_obj: tok::ExprAST = tok::ExprAST {Tokens: Vec::new()}; //TODO: Create seperate struct for extern
    //                         ast.push(Box::new(extern_obj));
    //                     }
    //                     _ => {
    //                         !unimplemented!();
    //                     }
    //                 }

    //                 identifier_str = String::from("");

    //             }

    //             else if valid_char.is_numeric() {   //NUMBER
    //                 identifier_str.push(valid_char);

    //                 while let Some(valid_char) = characters.next() {

    //                     if valid_char.is_numeric() {
    //                         identifier_str.push(valid_char);
    //                     }

    //                     else {
    //                         break;
    //                     }
    //                 }
                    
    //                 let num: tok::NumberExprAST = tok::NumberExprAST { Value: (identifier_str.parse().unwrap()) };

    //                 ast.push(Box::new(num)); //Storing num object in heap
    //                 identifier_str = String::from("");
    //             }

    //         } else { //EOF
    //             break;
    //         }
    //     }
        

    //     let parserExpr: tok::ExprAST = tok::ExprAST {
    //         Tokens: ast,
    //     };

    //     Parser {
    //         Root: parserExpr
    //     }
    }


}