use crate::{execute_function_call, Function, Value};
use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::HashMap;

/********************************* Methods ************************************/
pub fn assign_variable(
    var_name: &String,
    value: &Value,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) {
    match value {
        Value::Lambda(value) => {
            variables.insert(var_name.clone(), VarValue::from(value.clone()));
        }
        Value::FunctionCall(function) => {
            match execute_function_call(function, functions, variables) {
                Ok(Some(value)) => {
                    variables.insert(var_name.clone(), value);
                }
                Ok(None) => {
                    todo!("Cannot assign a function call without return value to a variable");
                }
                Err(_) => {
                    todo!("Function call failed");
                }
            }
        }
        Value::Variable(name) => {
            variables.insert(var_name.clone(), VarValue::clone_var(name, variables));
        }
    }
}

pub fn get_var_value(var_name: &String, variables: &HashMap<String, VarValue>) -> VarValue {
    match variables.get(var_name) {
        Some(var_value) => var_value.clone(),
        None => todo!("Variable {} not found", var_name),
    }
}

/****************************** VarType ***************************************/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VarType {
    Int,
    Float,
    String,
    Bool,
}

/****************************** VarValue **************************************/
pub struct VarValue {
    pub value: String,
    pub type_: VarType,
    pub is_type_enforced: bool,
}

impl Clone for VarValue {
    fn clone(&self) -> Self {
        VarValue {
            value: self.value.clone(),
            type_: self.type_.clone(),
            is_type_enforced: self.is_type_enforced,
        }
    }
}

impl VarValue {
    fn clone_var(var_name: &String, variables: &HashMap<String, VarValue>) -> VarValue {
        get_var_value(var_name, variables)
    }
}

impl From<bool> for VarValue {
    fn from(value: bool) -> Self {
        VarValue {
            value: value.to_string(),
            type_: VarType::Bool,
            is_type_enforced: true,
        }
    }
}

impl From<&str> for VarValue {
    fn from(s: &str) -> Self {
        let var_type: VarType = match s.parse::<i32>() {
            Ok(_) => VarType::Int,
            Err(_) => VarType::String,
        };
        let var_type = match s.parse::<f32>() {
            Ok(_) => VarType::Float,
            Err(_) => var_type,
        };
        VarValue {
            value: s.to_string(),
            type_: var_type,
            is_type_enforced: false,
        }
    }
}

impl From<String> for VarValue {
    fn from(s: String) -> Self {
        VarValue::from(s.as_str())
    }
}

impl Display for VarValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
