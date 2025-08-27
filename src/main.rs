use std::fs;

#[derive(Debug)]
enum Token {
    EOF,

    // Commands
    Def,                //['d', 'e', 'f']
    Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    Identifier,         //[a-zA-Z][a-zA-Z0-9]*
    Number              //[0-9]+
}

struct ExprAST {
    Type: String
    //NEED TO INCLUDE FIELD THAT HOLDS THE DATA FOR THE ACTUAL AST OBJECT
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

fn main() {
    let file_path = String::from("test_files/test1.txt");
    let contents = fs::read_to_string(file_path)
                    .expect("FILE NAME NOT VALID");

    let mut tokens: Vec<Token> = Vec::new();
    let mut identifier_str: String = String::from("");

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

    //START PARSER
    loop {
        Token currToken = get_token();

        //REFRAINING FROM USING MATCH CASE FOR CODE CLEANLINESS :|

        if (currToken == Token::EOF) { 
            break; 
        } 

        else if (currToken == Token::Number) {
                 
        }
    }

}
