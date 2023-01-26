#[allow(unused_imports)]
use crate::{execute_function_call, FunctionsMap, ValueGetter, VariablesMap};
use core::fmt::Display;
use core::fmt::Formatter;
use std::any::{Any, TypeId};

/********************************* Methods ************************************/
pub fn assign_variable(
    var_name: &String,
    value: &ValueGetter,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) {
    match value {
        ValueGetter::Lambda(value) => {
            variables
                .0
                .insert(var_name.clone(), Variable::from(value.clone()));
        }
        ValueGetter::FunctionCall(function) => {
            match execute_function_call(function, functions, variables) {
                Ok(Some(value)) => {
                    variables.0.insert(var_name.clone(), value);
                }
                Ok(None) => {
                    todo!("Cannot assign a function call without return value to a variable");
                }
                Err(_) => {
                    todo!("Function call failed");
                }
            }
        }
        ValueGetter::Variable(name) => {
            variables
                .0
                .insert(var_name.clone(), Variable::clone_var(name, variables));
        }
    }
}

pub fn get_var_value(var_name: &String, variables: &VariablesMap) -> Variable {
    match variables.0.get(var_name) {
        Some(var_value) => var_value.clone(),
        None => todo!("Variable {} not found", var_name),
    }
}

/****************************** ***************************************/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VarType {
    Int,
    Float,
    String,
    Bool,
    Tuple,
    Array,
}
impl Display for VarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            VarType::Int => write!(f, "int"),
            VarType::Float => write!(f, "float"),
            VarType::String => write!(f, "string"),
            VarType::Bool => write!(f, "bool"),
            VarType::Tuple => write!(f, "tuple"),
            VarType::Array => write!(f, "array"),
        }
    }
}

/****************************** **************************************/
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Tuple(Vec<Variable>),
    Array(Vec<Variable>),
}

impl Value {
    pub fn parse<T: Any>(&self) -> Result<T, ()> {
        if TypeId::of::<T>() == TypeId::of::<i32>() {
            match self {
                Value::Int(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<i32, T>(value));
                },
                Value::Float(value) => {
                    let f = value.round() as i32;
                    unsafe {
                        return Ok(std::mem::transmute_copy::<i32, T>(&f));
                    }
                }
                Value::String(value) => match value.parse::<i32>() {
                    Ok(value) => unsafe {
                        return Ok(std::mem::transmute_copy::<i32, T>(&value));
                    },
                    Err(_) => {
                        return Err(());
                    }
                },
                _ => todo!("Cannot parse {} to int", std::any::type_name::<T>()),
            }
        } else if TypeId::of::<T>() == TypeId::of::<f32>() {
            match self {
                Value::Float(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<f32, T>(value));
                },
                Value::Int(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<f32, T>(&(*value as f32)));
                },
                Value::String(value) => match value.parse::<f32>() {
                    Ok(value) => unsafe {
                        return Ok(std::mem::transmute_copy::<f32, T>(&value));
                    },
                    Err(_) => {
                        return Err(());
                    }
                },
                _ => todo!("Cannot parse value to float"),
            }
        } else if TypeId::of::<T>() == TypeId::of::<String>() {
            match self {
                Value::String(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<String, T>(value));
                },
                Value::Int(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<String, T>(&value.to_string()));
                },
                Value::Float(value) => unsafe {
                    return Ok(std::mem::transmute_copy::<String, T>(&value.to_string()));
                },
                _ => todo!("Cannot parse value to string"),
            }
        }
        todo!("Cannot parse value to type {}", std::any::type_name::<T>());
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Tuple(value) => {
                write!(f, "(")?;
                for (i, var) in value.iter().enumerate() {
                    write!(f, "{}", var)?;
                    if i != value.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Value::Array(value) => {
                write!(f, "[")?;
                for (i, var) in value.iter().enumerate() {
                    write!(f, "{}", var)?;
                    if i != value.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

/****************************** **************************************/
#[derive(Clone, Debug)]
pub struct Variable {
    pub value: Value,
    pub enforced_type: Option<VarType>,
}

impl Variable {
    fn clone_var(var_name: &String, variables: &VariablesMap) -> Variable {
        get_var_value(var_name, variables)
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Variable {
            value: Value::Bool(value),
            enforced_type: None,
        }
    }
}

impl From<i32> for Variable {
    fn from(value: i32) -> Self {
        Variable {
            value: Value::Int(value),
            enforced_type: None,
        }
    }
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        let var_type: VarType = match s.parse::<i32>() {
            Ok(_) => VarType::Int,
            Err(_) => VarType::String,
        };
        let var_type = match s.parse::<f32>() {
            Ok(_) => VarType::Float,
            Err(_) => var_type,
        };
        let var_type = match s.parse::<bool>() {
            Ok(_) => VarType::Bool,
            Err(_) => var_type,
        };
        Variable {
            value: match var_type {
                VarType::Int => Value::Int(s.parse::<i32>().unwrap()),
                VarType::Float => Value::Float(s.parse::<f32>().unwrap()),
                VarType::String => Value::String(s.to_string()),
                VarType::Bool => Value::Bool(s.parse::<bool>().unwrap()),
                _ => todo!("Tuple not implemented"),
            },
            enforced_type: Some(var_type),
        }
    }
}

impl From<String> for Variable {
    fn from(s: String) -> Self {
        let var_type: VarType = match s.parse::<i32>() {
            Ok(_) => VarType::Int,
            Err(_) => VarType::String,
        };
        let var_type = match s.parse::<f32>() {
            Ok(_) => VarType::Float,
            Err(_) => var_type,
        };
        let var_type = match s.parse::<bool>() {
            Ok(_) => VarType::Bool,
            Err(_) => var_type,
        };
        Variable {
            value: match var_type {
                VarType::Int => Value::Int(s.parse::<i32>().unwrap()),
                VarType::Float => Value::Float(s.parse::<f32>().unwrap()),
                VarType::String => Value::String(s),
                VarType::Bool => Value::Bool(s.parse::<bool>().unwrap()),
                _ => todo!("Tuple not implemented"),
            },
            enforced_type: Some(var_type),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Tuple(value) => {
                write!(f, "(")?;
                for (i, var) in value.iter().enumerate() {
                    write!(f, "{}", var)?;
                    if i != value.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Value::Array(value) => {
                write!(f, "[")?;
                for (i, var) in value.iter().enumerate() {
                    write!(f, "{}", var)?;
                    if i != value.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}
