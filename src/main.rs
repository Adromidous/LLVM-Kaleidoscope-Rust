use std::fs;

#[derive(Debug)]
enum Token {
    EOF,

    // Commands
    Def,                //['d', 'e', 'f']
    Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    Identifier,         //[a-zA-Z][a-zA-Z0-9]*
    Number,             //[0-9]+

    // Miscellaneous
    Whitespace,
    Error
}

struct ExprAST {
    Type: String
}

struct VariableExprAST {
    Name: String
}

struct NumberExprAST {
    Val: u32
}

struct BinaryExprAST {
    Operator: char,
    LHS: ExprAST,
    RHS: ExprAST
}

struct UnaryExprAST {
    Operator: char,
    Var: ExprAST
}

struct CallerExprAST { //Expression struct for function calls
    Callee: String,
    Arguments: Vec<ExprAST>
}

struct PrototypeExprAST { //Expression struct for function definitions 
    Name: String,
    Arguments: Vec<String>
}

struct FunctionExprAST {
    Prototype: PrototypeExprAST, //Function name
    Body: ExprAST
}

struct Lexer {
    identifier_str: String,
    num_val: u32,
    contents: String,
    tokens: Vec<Token>
}

impl Lexer {

    fn new(filename: &String) -> Lexer {
        let mut contents = fs::read_to_string(filename)
                    .expect("FILE NAME NOT VALID");

        let mut tokens = Vec::new();
        let mut identifier_str = String::from("");

        let mut characters = contents.chars();

        loop {
            let character: Option<char> = characters.next();

            if let Some(valid_char) = character {

                if valid_char == ' ' { //WHITESPACES
                    continue;
                }

                else if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
                    identifier_str.push(valid_char);
                    
                    while let Some(valid_char) = characters.next() {
                        
                        if valid_char.is_alphabetic() || valid_char.is_numeric() {
                            identifier_str.push(valid_char);    
                        }

                        else {
                            break;
                        }

                    }

                    match identifier_str.as_str() {
                        "def" => tokens.push(Token::Def),
                        "extern" => tokens.push(Token::Extern),
                        _ => tokens.push(Token::Identifier),
                    }

                    identifier_str = String::from("");

                } 

                else if valid_char.is_numeric() {   //NUMBER
                    identifier_str.push(valid_char);

                    while let Some(valid_char) = characters.next() {

                        if valid_char.is_numeric() {
                            identifier_str.push(valid_char);
                        }

                        else {
                            break;
                        }
                    }
                    
                    tokens.push(Token::Number);
                    identifier_str = String::from("");
                }

            } else { //EOF
                tokens.push(Token::EOF);
                break;
            }

        }
    
        Lexer { 
            identifier_str: identifier_str,
            num_val: 0,
            contents: contents,
            tokens: tokens 
        }

    }

    fn print_tokens(&self){
        for tok in self.tokens.iter() {
            println!("TOKEN: {:#?}", tok);
        }
    }

}

struct Parser {
    identifier_str: String,
    num_val: u32,
    contents: String,
}

impl Parser {

    fn new(filename: &String) -> Self {
        let mut contents = fs::read_to_string(filename)
                            .expect("FILE NAME NOT VALID");

        let mut identifier_str = String::from("");

        Parser {
            identifier_str: identifier_str,
            num_val: 0,
            contents: contents,
        }
    }

    fn gettoken(&mut self) -> Token {
        self.identifier_str = String::from("");

        let mut characters = self.contents.chars();

        let character: Option<char> = characters.next();

        if let Some(valid_char) = character {

            if valid_char == ' ' { //WHITESPACES
                return Token::Whitespace; 
            }

            else if valid_char.is_alphabetic() { //DEF, EXTERN, IDENTIFIER
                self.identifier_str.push(valid_char);
                
                while let Some(valid_char) = characters.next() {
                    
                    if valid_char.is_alphabetic() || valid_char.is_numeric() {
                        self.identifier_str.push(valid_char);    
                    }

                    else {
                        break;
                    }

                }

                match self.identifier_str.as_str() {
                    "def" => return Token::Def,
                    "extern" => return Token::Extern,
                    _ => return Token::Identifier,
                }


            } 

            else if valid_char.is_numeric() {   //NUMBER
                self.identifier_str.push(valid_char);

                while let Some(valid_char) = characters.next() {

                    if valid_char.is_numeric() {
                        self.identifier_str.push(valid_char);
                    }

                    else {
                        break;
                    }
                }
                
                let store_num = self.identifier_str.parse();

                match store_num {
                    Ok(val) => self.num_val = val,
                    _ => println!("EXPECTED NUMBER!")
                }

                return Token::Number;
            }

            else { //EOF
                return Token::EOF;
            }
        } 

        else {
            return Token::Error;
        }
    }

   fn retAST(&mut self) -> ExprAST {
   }

}

fn main() {
    let file_path = String::from("test_files/test1.txt");

    let mut lex1 = Lexer::new(&file_path);

    lex1.print_tokens();

    let mut parser1 = Parser::new(&file_path);
}
