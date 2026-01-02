use std::ops::Deref;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Commands
    //Def,                //['d', 'e', 'f']
    //Extern,             //['e', 'x', 't', 'e', 'r', 'n']

    // Primary
    Identifier,         //[a-zA-Z][a-zA-Z0-9]*
    Number,             //[0-9]+
    Operator,           //['+', '-', '*', '/', '=']

    // Miscellaneous
    Whitespace,
    EOF
    //Error
}

pub trait Visit {
    fn print(&self);
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
}

pub struct VariableExprAST {
    pub name: String,
}

impl Visit for VariableExprAST {
    fn print(&self) {
        println!("{}", self.name);
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
}

impl Deref for BinaryExprAST {
    type Target = BinaryExprAST;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

// pub struct CallExprAST {
//     pub Callee: String,
//     pub Args: ExprAST,
// }

// impl Visit for CallExprAST {
//     fn print(&self) {
//         println!("{}", self.Callee);
//         self.Args.print();
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