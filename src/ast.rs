use crate::{execute_function_call, get_var_value, HashMap, VarValue};

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
    FunctionCall(Function),
    Variable(String),
    Lambda(String),
}

impl Value {
    pub fn get_value(
        &self,
        variables: &mut HashMap<String, VarValue>,
        functions: &mut HashMap<String, Function>,
    ) -> VarValue {
        return match self {
            Value::Variable(name) => get_var_value(name, variables),
            Value::Lambda(value) => VarValue::from(value.clone()),
            Value::FunctionCall(function) => {
                match execute_function_call(function, functions, variables) {
                    Some(value) => value,
                    None => {
                        panic!("Function call failed");
                    }
                }
            }
        };
    }
}

pub struct Function {
    pub name: String,
    pub args: Vec<Value>,
    pub body: Option<Vec<AstNode>>,
}

pub enum ControlFlow {
    WhileLoop {
        fn_condition: Function,
        body: Sequence,
    },
}
