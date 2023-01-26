use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::HashMap;

/********************************* Methods ************************************/
pub fn assign_var(
    var_name: &VarName,
    var_value: &VarValue,
    variables: &mut HashMap<VarName, VarValue>,
) {
    variables.insert(var_name.clone(), var_value.clone());
    if variables.contains_key(&VarName::from("i")) {
        println!("i is {}", variables.get(&VarName::from("i")).unwrap());
    }
}

pub fn get_var_value(var_name: &VarName, variables: &HashMap<VarName, VarValue>) -> VarValue {
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

/****************************** VarName ***************************************/
#[derive(PartialEq, Eq, Hash)]
pub struct VarName(pub String);
impl From<&str> for VarName {
    fn from(s: &str) -> Self {
        VarName(s.to_string())
    }
}

impl Display for VarName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Clone for VarName {
    fn clone(&self) -> Self {
        VarName(self.0.clone())
    }
}
