use std::fs;

enum Token {
    EOF,

    // Commands
    Def,                //['d', 'e', 'f']
    Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    Identifier,         //[a-zA-Z][a-zA-Z0-9]*
    Number              //[0-9]+
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
                
                loop {
                    let character = characters.next();
                    match character {
                        Some(val) => identifier_str.push(val),
                        None => break 
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

                loop {
                    let character = characters.next();
                    match character {
                        Some(val) => identifier_str.push(val),
                        None => break
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

}
