#[allow(unused_imports)]
use crate::{execute_function_call, get_var_value, HashMap, Variable};

#[derive(Clone)]
pub enum AstNode {
    VariableAssignment { name: String, value: ValueGetter },
    ControlFlow(ControlFlow),
    FunctionDeclaration(FunctionSignature),
    FunctionCall(FunctionCall),
}

#[derive(Clone)]
pub struct Sequence {
    pub sequence: Vec<AstNode>,
}

#[derive(Clone)]
pub enum ValueGetter {
    FunctionCall(FunctionCall),
    Variable(String),
    Lambda(String),
}

pub struct FunctionsMap(pub HashMap<String, FunctionSignature>);
pub struct VariablesMap(pub HashMap<String, Variable>);

impl ValueGetter {
    pub fn get_value(
        &self,
        functions: &mut FunctionsMap,
        variables: &mut VariablesMap,
    ) -> Variable {
        return match self {
            ValueGetter::Variable(name) => get_var_value(name, variables),
            ValueGetter::Lambda(value) => Variable::from(value.clone()),
            ValueGetter::FunctionCall(function) => {
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
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<ValueGetter>,
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub args: HashMap<String, Option<Variable>>,
    pub body: Sequence,
}

#[derive(Clone)]
pub enum ControlFlow {
    WhileLoop {
        fn_condition: FunctionCall,
        body: Sequence,
    },
}
