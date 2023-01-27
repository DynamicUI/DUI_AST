#[allow(unused_imports)]
use super::{FunctionCall, FunctionsMap, Value, VarType, Variable, VariablesMap};

#[allow(dead_code, unused_variables)]
pub fn comparaison(left: Variable, operator: &str, right: Variable) -> bool {
    match (left.enforced_type, right.enforced_type) {
        (Some(left_type), Some(right_type)) => {
            if left_type != right_type {
                todo!("Type mismatch: {:?} and {:?}", left_type, right_type);
            }
        }
        (Some(left_type), None) => {}
        (None, Some(right_type)) => {}
        (None, None) => {}
    }

    match operator {
        "less_than" => match left.value {
            Value::Int(left_value) => left_value < right.value.parse::<i32>().unwrap(),
            Value::Float(left_value) => left_value < right.value.parse::<f32>().unwrap(),
            Value::String(left_value) => todo!("Comparaison between strings"),
            Value::Bool(left_value) => todo!("Comparaison between booleans"),
            Value::Tuple(left_value) => todo!("Comparaison between tuples"),
            Value::Array(left_value) => todo!("Comparaison between arrays"),
        },
        _ => false,
    }
}

#[allow(dead_code, unused_variables)]
pub fn add(left: &Variable, right: &Variable) -> Variable {
    match (&left.enforced_type, &right.enforced_type) {
        (Some(left_type), Some(right_type)) => {
            if left_type != right_type {
                //todo!("Type mismatch: {:?} and {:?}", left_type, right_type);
            }
        }
        (Some(left_type), None) => {}
        (None, Some(right_type)) => {}
        (None, None) => {}
    }
    match &left.value {
        Value::Int(left_value) => Variable {
            value: Value::Int(left_value + right.value.parse::<i32>().unwrap()),
            enforced_type: Some(VarType::Int),
        },
        Value::Float(left_value) => Variable {
            value: Value::Float(left_value + right.value.parse::<f32>().unwrap()),
            enforced_type: Some(VarType::Float),
        },
        Value::String(left_value) => Variable {
            value: Value::String(format!("{}{}", left_value, right.value)),
            enforced_type: Some(VarType::String),
        },
        Value::Bool(left_value) => todo!("Addition between booleans"),
        Value::Tuple(left_value) => todo!("Addition between tuples"),
        Value::Array(left_value) => todo!("Addition between arrays"),
    }
}

const NATIVE_FUNCTIONS_NAMES: [&str; 9] = [
    "add",
    "less_than",
    "println",
    "print",
    "equal_than",
    "greater_than",
    "less_or_equal_than",
    "greater_or_equal_than",
    "not_equal_than",
];

pub fn is_native_function(name: &str) -> bool {
    NATIVE_FUNCTIONS_NAMES.contains(&name)
}

pub fn execute_native_function(
    function: &FunctionCall,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) -> Result<Option<Variable>, ()> {
    match &function.name as &str {
        "print" | "println" => {
            for arg in function.args.iter() {
                let value = arg.get_value(functions, variables);
                print!("{}", value);
            }
            if function.name == "println" {
                println!();
            }
            return Ok(None);
        }
        "less_than"
        | "equal_than"
        | "greater_than"
        | "less_or_equal_than"
        | "greater_or_equal_than"
        | "not_equal_than" => {
            if function.args.len() != 2 {
                todo!("{} function takes 2 arguments", function.name);
            }
            let left = function.args[0].get_value(functions, variables);
            let right = function.args[1].get_value(functions, variables);
            return Ok(Some(Variable::from(comparaison(
                left,
                &function.name,
                right,
            ))));
        }
        "add" => {
            if function.args.len() != 2 {
                todo!(
                    "add function takes 2 arguments (and not {})",
                    function.args.len()
                );
            }
            let left = function.args[0].get_value(functions, variables);
            let right = function.args[1].get_value(functions, variables);
            return Ok(Some(Variable::from(add(&left, &right))));
        }
        _ => {}
    }

    Err(())
}
