use crate::{VarType, VarValue};

pub enum ComparaisonOperator {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

pub fn comparaison(left: VarValue, operator: ComparaisonOperator, right: VarValue) -> bool {
    if left.type_ != right.type_ {
        panic!("Comparaison between different types");
    }
    match operator {
        ComparaisonOperator::LessThan => match left.type_ {
            VarType::Int => {
                left.value.parse::<i32>().unwrap() < right.value.parse::<i32>().unwrap()
            }
            VarType::Float => {
                left.value.parse::<f32>().unwrap() < right.value.parse::<f32>().unwrap()
            }
            VarType::String => panic!("Cannot compare strings"),
        },
        _ => false,
    }
}

pub fn add(left: &VarValue, right: &VarValue) -> VarValue {
    if left.type_ != right.type_ {
        panic!("Cannot add different types");
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
    }
}
