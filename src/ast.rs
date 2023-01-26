use crate::{execute_function_call, get_var_value, HashMap, VarValue};

#[derive(Clone)]
pub enum AstNode {
    VariableAssignment { name: String, value: Value },
    ControlFlow(ControlFlow),
    FunctionDeclaration(Function),
    FunctionCall(Function),
}

#[derive(Clone)]
pub struct Sequence {
    pub sequence: Vec<AstNode>,
}

#[derive(Clone)]
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
                    Ok(Some(value)) => value,
                    Ok(None) => {
                        todo!("Cannot assign a function call without return value to a variable")
                    }
                    Err(_) => todo!("Function call failed"),
                }
            }
        };
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Value>,
    pub body: Option<Sequence>,
}

#[derive(Clone)]
pub enum ControlFlow {
    WhileLoop {
        fn_condition: Function,
        body: Sequence,
    },
}
