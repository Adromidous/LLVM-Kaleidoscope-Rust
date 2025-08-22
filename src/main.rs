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

        if character == None {
            break;
        }

        identifier_str.push(character);

        if character == Some(' ') {
            identifier_str = String::from("");
        }
    }

    tokens.push(Token::EOF);
}
