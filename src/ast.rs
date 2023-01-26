pub enum AstNode {
    VariableAssignment { name: String, value: Value },
    ControlFlow(ControlFlow),
    FunctionDeclaration(Function),
    FunctionCall(Function),
}

pub struct Sequence {
    pub sequence: Vec<AstNode>,
}

pub enum Value {
    FunctionCall { name: String, args: Vec<Value> },
    Variable(String),
    Lambda(String),
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Option<Vec<AstNode>>,
}

pub enum ControlFlow {
    WhileLoop {
        fn_condition: Function,
        body: Sequence,
    },
}
