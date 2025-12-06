#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
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

pub struct ExprAST {
    pub Tokens: Vec<Token>    
}

pub struct VariableExprAST {
    pub Name: String,
    pub Value: NumberExprAST
}

pub struct NumberExprAST {
    pub Value: usize
}

pub struct BinaryExprAST {
    pub Operator: char,
    pub LHS: VariableExprAST,
    pub RHS: NumberExprAST
}

pub struct UnaryExprAST {
    pub Operator: char,
    pub Name: VariableExprAST
}