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

pub trait ToExpr {
    fn convert(&self) -> ExprAST;
}

pub struct ExprAST {
    pub Children: Vec<Box<dyn Visit>>,
}

impl Visit for ExprAST {
    fn print(&self) {
        for tok in self.Children.iter() {
            tok.print();
        }
    }
}

pub struct VariableExprAST {
    pub Name: String,
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
    pub LHS: Box<dyn Visit>,
    pub RHS: Box<dyn Visit>,
}

impl Visit for BinaryExprAST {
    fn print(&self) {
        println!("{}", self.Operator);
    }
}

pub struct CallExprAST {
    pub Callee: String,
    pub Args: ExprAST,
}

impl Visit for CallExprAST {
    fn print(&self) {
        println!("{}", self.Callee);
        for tok in self.Args.Children.iter() {
            tok.print();
        }
    }
}

pub struct PrototypeAST { //Captures the function declaration - Name and arguments of a function
    pub Name: String,
    pub Args: Vec<Box<dyn Visit>>,
}

pub struct FunctionAST { //Captures the function definition itself
    pub Proto: PrototypeAST,
    pub Body: ExprAST,
}