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

                println!("Identifier: {0}", identifier_str);
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
                
                println!("Number: {0}", identifier_str);
                tokens.push(Token::Number);
                identifier_str = String::from("");
            }

        } else { //EOF
            tokens.push(Token::EOF);
            break;
        }

    }

    for token in tokens.iter() {
        println!("TOKEN: {:#?}", token);
    }
}
