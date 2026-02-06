use std::ops::Deref;
use std::{fs::File, io::Write};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Commands
    // Def,                //['d', 'e', 'f']
    // Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    IDENTIFIER,         //[a-zA-Z][a-zA-Z0-9]*
    NUMBER,             //[0-9]+
    OPERATOR,           //['+', '-', '*', '/']
    EQUAL,              //['=']

    OPENPARENT,
    CLOSEPARENT,
    WHITESPACE,
    MISC,               //['\t', '\n']
    EOF                 //['\0']
}

// pub trait Visit {
//     fn print(&self);
// }

// pub struct ExprAST {
//     pub child: Box<dyn Visit>,
// }

// impl Visit for ExprAST {
//     fn print(&self) {
//         self.print();
//     }
// }

// pub struct VariableExprAST {
//     pub name: String,
// }

// impl Visit for VariableExprAST {
//     fn print(&self) {
//         println!("{}", self.name);
//     }
// }

// pub struct NumberExprAST {
//     pub value: usize
// }

// impl Visit for NumberExprAST {
//     fn print(&self) {
//         println!("{}", self.value);
//     }
// }

// pub struct UnaryExprAST {
//     pub negate: bool,
//     pub child: Box<dyn Visit>
// }

// impl Visit for UnaryExprAST {
//     fn print(&self) {
//         print!("-");
//         self.child.print();
//     }
// }

// pub struct BinaryExprAST {
//     pub operator: String,
//     pub lhs: Box<dyn Visit>,
//     pub rhs: Box<dyn Visit>,
// }

// impl Visit for BinaryExprAST {
//     fn print(&self) {
//         self.lhs.print();
//         println!("{}", self.operator);
//         self.rhs.print();
//     }
// }

// pub struct EqualExprAST {
//     pub lhs: VariableExprAST,
//     pub rhs: Box<dyn Visit>,
// }

// impl Visit for EqualExprAST {
//     fn print(&self) {
//         println!("{}", self.lhs.name);
//         println!("=");
//         self.rhs.print();
//     }
// }

// pub struct ParenthesisExprAST {
//     pub child: Box<dyn Visit>
// }

// impl Visit for ParenthesisExprAST {
//     fn print(&self) {
//         self.child.print();
//     }
// }

// pub struct EOFExprAST {
// }

// impl Visit for EOFExprAST {
//     fn print(&self) {
//         println!("EOF");
//     }
// }

// pub struct CallExprAST {
//     pub Callee: String,
//     pub Args: Vec<Box<dyn Visit>>,
// }

// impl Visit for CallExprAST {
//     fn print(&self) {
//         println!("{}", self.Callee);
        
//         for arg in self.Args.into_iter() {
//             arg.print();
//         }
//     }
// }

// pub struct PrototypeAST { //Captures the function declaration - Name and arguments of a function
//     pub Name: String,
//     pub Args: Vec<Box<dyn Visit>>,
// }

// pub struct FunctionAST { //Captures the function definition itself
//     pub Proto: PrototypeAST,
//     pub Body: ExprAST,
// }