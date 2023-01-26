use crate::{execute_function_call, Value};
use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::HashMap;

/********************************* Methods ************************************/
pub fn assign_variable(
    var_name: &String,
    value: &Value,
    variables: &mut HashMap<String, VarValue>,
) {
    match value {
        Value::Lambda(value) => {
            variables.insert(var_name.clone(), VarValue::from(value.clone()));
        }
        Value::FunctionCall { name, args } => match execute_function_call(name, args, variables) {
            Some(value) => {
                variables.insert(var_name.clone(), value);
            }
            None => {
                panic!("Function call failed");
            }
        },
        Value::Variable(name) => {
            variables.insert(var_name.clone(), VarValue::clone_var(name, variables));
        }
    }
}

pub fn get_var_value(var_name: &String, variables: &HashMap<String, VarValue>) -> VarValue {
    match variables.get(var_name) {
        Some(var_value) => var_value.clone(),
        None => panic!("Variable {} not found", var_name),
    }
}

/****************************** VarType ***************************************/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VarType {
    Int,
    Float,
    String,
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
