use std::collections::HashMap;
use std::collections::LinkedList;

mod ast;
mod blocks;
mod variables;
use ast::{AstResult, Block, Op, Var, AST};
use blocks::Condition;
use variables::{VarName, VarType, VarValue};

fn main() {
    let root_sequence: LinkedList<AST> = LinkedList::from([
        AST::Var(Var::Assign(VarName::from("i"), VarValue::from("1"))),
        AST::Block(Block::While(
            Condition {
                left: Var::Ref(VarName::from("i")),
                right: Var::Lambda(VarValue::from("10")),
                op: Op::LessThan,
            },
            LinkedList::from([AST::Op(Op::Add(
                Var::Ref(VarName::from("i")),
                Var::Lambda(VarValue::from("1")),
            ))]),
        )),
        AST::Func("print(i)".to_string()),
    ]);

    let mut variables: HashMap<VarName, VarValue> = HashMap::new();
    execute(&root_sequence, &mut variables);
    //recursive_var_change(5, &mut variables);
}

fn evaluate_condition(condition: &Condition, left: &VarValue, right: &VarValue) -> bool {
    if left.type_ != right.type_ {
        panic!("Cannot compare different types");
    }
    match condition.op {
        Op::LessThan => match left.type_ {
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

fn assign_var(
    var_name: &VarName,
    var_value: &VarValue,
    variables: &mut HashMap<VarName, VarValue>,
) {
    variables.insert(var_name.clone(), var_value.clone());
    if variables.contains_key(&VarName::from("i")) {
        println!("i is {}", variables.get(&VarName::from("i")).unwrap());
    }
}

fn get_var_value(var: &Var, variables: &HashMap<VarName, VarValue>) -> VarValue {
    match var {
        Var::Lambda(var_value) => var_value.clone(),
        Var::Ref(var_name) => match variables.get(var_name) {
            Some(var_value) => var_value.clone(),
            None => panic!("Variable {} not found", var_name),
        },
        _ => panic!("Cannot get value from a variable assignment"),
    }
}

fn add_var_values(left: &VarValue, right: &VarValue) -> VarValue {
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

fn execute(sequence: &LinkedList<AST>, variables: &mut HashMap<VarName, VarValue>) -> AstResult {
    for ast in sequence {
        execute_ast(ast, variables);
    }
    AstResult::None
}

fn execute_ast(ast: &AST, variables: &mut HashMap<VarName, VarValue>) -> AstResult {
    match ast {
        AST::Var(var) => match var {
            Var::Assign(var_name, var_value) => {
                assign_var(var_name, var_value, variables);
                return AstResult::None;
            }
            Var::Lambda(var_value) => {
                println!("Lambda not used: {}", var_value);
                return AstResult::VarValue(var_value.clone());
            }
            Var::Ref(var_name) => match variables.get(&var_name) {
                Some(var_value) => return AstResult::VarValue(var_value.clone()),
                None => return AstResult::NoVarFound,
            },
        },
        AST::Block(block) => match block {
            Block::While(condition, inner_sequence) => {
                let mut i = 0;
                while evaluate_condition(
                    &condition,
                    &get_var_value(&condition.left, &variables),
                    &get_var_value(&condition.right, &variables),
                ) {
                    execute(&inner_sequence, variables);
                    if i >= 100 {
                        println!("Infinite loop detected ! (more than {}, see in parameters to change the max)", i);
                        break;
                    }
                    i += 1;
                }
            } // Ne retourne rien
        },
        AST::Func(func) => {
            println!("Func: {}", func);
            // TODO return result
        }
        AST::Op(op) => match op {
            Op::Add(left, right) => {
                let i = add_var_values(
                    &get_var_value(left, &variables),
                    &get_var_value(right, &variables),
                );
                return AstResult::VarValue(i);
            }
            Op::LessThan => panic!("Cannot use less than in an operation"),
        },
    }
    AstResult::None
}
