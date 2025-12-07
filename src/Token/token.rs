use std::ops::Deref;

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
    Operator,           //['+', '-', '*', '/']

    // Miscellaneous
    Whitespace,
    Error
}

pub trait Visit {
    fn print(&self);
}

pub struct ExprAST {
    pub Tokens: Vec<Box<dyn Visit>>,
}

impl Visit for ExprAST {
    fn print(&self) {
        for tok in self.Tokens.iter() {
            tok.print();
        }
    }
}

pub struct VariableExprAST {
    pub Name: String,
    pub Value: NumberExprAST
}

impl Visit for VariableExprAST {
    fn print(&self) {
        println!("{}", self.Name);
    }
}

pub struct NumberExprAST {
    pub Value: usize
}

impl Visit for NumberExprAST {
    fn print(&self) {
        println!("{}", self.Value);
    }
}

pub struct BinaryExprAST {
    pub Operator: String,
    pub LHS: NumberExprAST,
    pub RHS: NumberExprAST,
}

impl Visit for BinaryExprAST {
    fn print(&self) {
        println!("{}", self.Operator);
    }
}