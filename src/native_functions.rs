use crate::{Function, HashMap, VarType, VarValue};

pub fn comparaison(left: VarValue, operator: &str, right: VarValue) -> bool {
    if left.type_ != right.type_ {
        todo!("Comparaison between different types");
    }
    match operator {
        "less_than" => match left.type_ {
            VarType::Int => {
                left.value.parse::<i32>().unwrap() < right.value.parse::<i32>().unwrap()
            }
            VarType::Float => {
                left.value.parse::<f32>().unwrap() < right.value.parse::<f32>().unwrap()
            }
            VarType::String => todo!("Cannot compare strings"),
            VarType::Bool => todo!("Cannot compare booleans"),
        },
        _ => false,
    }
}

pub fn add(left: &VarValue, right: &VarValue) -> VarValue {
    if left.type_ != right.type_ {
        //println!("{} {}", left.type_, right.type_);
        //todo!("Cannot add different types");
    }
    match left.type_ {
        VarType::Int => VarValue {
            value: (left.value.parse::<i32>().unwrap() + right.value.parse::<i32>().unwrap())
                .to_string(),
            type_: VarType::Int,
            is_type_enforced: false,
        },
        VarType::Float => VarValue {
            value: (left.value.parse::<f32>().unwrap() + right.value.parse::<f32>().unwrap())
                .to_string(),
            type_: VarType::Float,
            is_type_enforced: false,
        },
        VarType::String => VarValue {
            value: format!("{}{}", left.value, right.value),
            type_: VarType::String,
            is_type_enforced: false,
        },
        VarType::Bool => todo!("Cannot add booleans"),
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
    function: &Function,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) -> Result<Option<VarValue>, ()> {
    match &function.name as &str {
        "print" | "println" => {
            for arg in function.args.iter() {
                let value = arg.get_value(variables, functions);
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
            let left = function.args[0].get_value(variables, functions);
            let right = function.args[1].get_value(variables, functions);
            return Ok(Some(VarValue::from(comparaison(
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
            let left = function.args[0].get_value(variables, functions);
            let right = function.args[1].get_value(variables, functions);
            return Ok(Some(VarValue::from(add(&left, &right))));
        }
        _ => {}
    }

    Err(())
}
