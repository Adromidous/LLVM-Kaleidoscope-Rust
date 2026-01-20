use std::ops::Deref;
use std::{fs::File, io::Write};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Commands
    Def,                //['d', 'e', 'f']
    Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    Identifier,         //[a-zA-Z][a-zA-Z0-9]*
    Number,             //[0-9]+
    Operator,           //['+', '-', '*', '/']
    Equal,              //['=']
    OpenParen,          //['(']
    CloseParen,         //[')']
    Comma,              //[',']

    // Miscellaneous
    Whitespace,         //[' ']
    EOF                 //['\0']
}

pub trait Visit {
    fn print(&self);
    fn write_instruction(&self, file: &mut File);
}

pub struct ExprAST {
    pub child: Vec<Box<dyn Visit>>,
}

impl Visit for ExprAST {
    fn print(&self) {
        for tok in self.child.iter() {
            tok.print();
        }
    }

    fn write_instruction(&self, file: &mut File) {
        todo!("Need to implement write instruction for ExprAST");
    }
}

pub struct VariableExprAST {
    pub name: String,
}

impl Visit for VariableExprAST {
    fn print(&self) {
        println!("{}", self.name);
    }

    fn write_instruction(&self, file: &mut File) {
        todo!("Need to implement write instruction for VariableExprAST");
    }
}

impl Deref for VariableExprAST {
    type Target = VariableExprAST;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

pub struct NumberExprAST {
    pub value: usize
}

impl Visit for NumberExprAST {
    fn print(&self) {
        println!("{}", self.value);
    }

    fn write_instruction(&self, file: &mut File) {
        file.write_all(&self.value.to_string().as_bytes());
    }
}

impl Deref for NumberExprAST {
    type Target = NumberExprAST;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

pub struct BinaryExprAST {
    pub operator: String,
    pub lhs: Box<dyn Visit>,
    pub rhs: Box<dyn Visit>,
}

impl Visit for BinaryExprAST {
    fn print(&self) {
        self.lhs.print();
        println!("{}", self.operator);
        self.rhs.print();
    }

    fn write_instruction(&self, file: &mut File) {
        match self.operator.as_str() {
            "+" => {
                file.write_all(b"ADD EAX, ");
                self.lhs.write_instruction(file);
                file.write_all(b"\n");

                file.write_all(b"ADD EAX, ");
                self.rhs.write_instruction(file);
                file.write_all(b"\n");
            }

            "-" => {
                file.write_all(b"SUB EAX, ");
                self.lhs.write_instruction(file);
                file.write_all(b"\n");

                file.write_all(b"SUB EAX, ");
                self.rhs.write_instruction(file);
                file.write_all(b"\n");
            },

            //TODO: IMPLEMENT MULTIPLY AND DIVIDE x86 INSTRUCTIONS

            _ => {},
        }
    }
}

impl Deref for BinaryExprAST {
    type Target = BinaryExprAST;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

pub struct EqualExprAST {
    pub lhs: VariableExprAST,
    pub rhs: Box<dyn Visit>,
}

impl Visit for EqualExprAST {
    fn print(&self) {
        println!("{}", self.lhs.name);
        println!("=");
        self.rhs.print();
    }

    fn write_instruction(&self, file: &mut File) {
        todo!("Need to implement write instruction for EqualExprAST");
    }
}

impl Deref for EqualExprAST {
    type Target = EqualExprAST;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

pub struct ParenthesisExprAST {
    pub child: Box<dyn Visit>
}

impl Visit for ParenthesisExprAST {
    fn print(&self) {
        self.child.print();
    }

    fn write_instruction(&self, file: &mut File) {
        todo!("Need to implement write instruction for ParenthesisExprAST");
    }
}

pub struct EOFExprAST {
}

impl Visit for EOFExprAST {
    fn print(&self) {
        println!("EOF");
    }

    fn write_instruction(&self, file: &mut File) {
        // file.write_all(b"ret");
        todo!("Need to implement write instruction for EOFExprAST");
    }
}

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