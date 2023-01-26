pub enum AstNode {
    VariableAssignment {
        name: String,
        value: Box<AstNode>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<AstNode>,
    },
    FunctionCall {
        name: String,
        parameters: Vec<AstNode>,
    },
    ControlFlow {
        condition_function_name: String,
        body: Vec<AstNode>,
    },
}
impl AstNode {}

pub struct Sequence {
    pub sequence: Vec<AstNode>,
}
