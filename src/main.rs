mod ast;
mod control_flows;
mod native_functions;
mod variables;

use ast::{AstNode, ControlFlow, Function, Sequence, Value};
use control_flows::while_loop;
use native_functions::{add, less_than};
use std::collections::HashMap;
use variables::{get_var_value, VarType, VarValue};

fn main() {
    let mut functions: HashMap<String, Function> = HashMap::new();
    let mut variables: HashMap<String, VarValue> = HashMap::new();
    let sequence = Sequence {
        sequence: vec![
            AstNode::VariableAssignment {
                name: "i".to_string(),
                value: Value::Lambda("0".to_string()),
            },
            AstNode::ControlFlow(ControlFlow::WhileLoop {
                fn_condition: Function {
                    name: "less_than".to_string(),
                    args: vec![
                        Value::Variable("i".to_string()),
                        Value::Lambda("10".to_string()),
                    ],
                    body: None,
                },
                body: Sequence {
                    sequence: vec![
                        AstNode::VariableAssignment {
                            name: "i".to_string(),
                            value: Value::FunctionCall(Function {
                                name: "add".to_string(),
                                args: vec![
                                    Value::Variable("i".to_string()),
                                    Value::Lambda("1".to_string()),
                                ],
                                body: None,
                            }),
                        },
                        AstNode::FunctionCall(Function {
                            name: "print".to_string(),
                            args: vec![Value::Variable("i".to_string())],
                            body: None,
                        }),
                    ],
                },
            }),
            AstNode::FunctionCall(Function {
                name: "print".to_string(),
                args: vec![Value::Lambda("Fin de la boucle !".to_string())],
                body: None,
            }),
        ],
    };

    execute_sequence(&sequence, &mut functions, &mut variables);
}

fn execute_ast_node(
    node: &AstNode,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) {
    match node {
        AstNode::VariableAssignment { name, value } => {
            variables::assign_variable(name, value, functions, variables);
        }
        AstNode::ControlFlow(control_flow) => match control_flow {
            ControlFlow::WhileLoop { fn_condition, body } => {
                while_loop(fn_condition, body, functions, variables);
            }
        },
        AstNode::FunctionCall(function) => {
            execute_function_call(function, functions, variables);
        }
        AstNode::FunctionDeclaration(function) => {
            //variables::declare_function(name, parameters, body);
        }
    }
}

fn execute_sequence(
    sequence: &Sequence,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) {
    for node in sequence.sequence.iter() {
        execute_ast_node(node, functions, variables);
    }
}

fn execute_function_call(
    function: &Function,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) -> Option<VarValue> {
    match &function.name as &str {
        "print" => {
            if function.args.len() == 0 {
                println!();
            }
            for arg in function.args.iter() {
                let value = arg.get_value(variables, functions);
                match value.type_ {
                    VarType::String => {
                        println!("{}", value);
                    }
                    VarType::Int => {
                        println!("{}", value);
                    }
                    VarType::Float => {
                        println!("{}", value);
                    }
                    VarType::Bool => {
                        println!("{}", value);
                    }
                }
            }
            return None;
        }
        "less_than" => {
            if function.args.len() != 2 {
                panic!("less_than function takes 2 arguments");
            }
            let left = function.args[0].get_value(variables, functions);
            let right = function.args[1].get_value(variables, functions);
            return Some(VarValue::from(less_than(left, right)));
        }
        "add" => {
            if function.args.len() != 2 {
                panic!(
                    "add function takes 2 arguments (and not {})",
                    function.args.len()
                );
            }
            let left = function.args[0].get_value(variables, functions);
            let right = function.args[1].get_value(variables, functions);
            return Some(VarValue::from(add(&left, &right)));
        }
        _ => {}
    }

    if functions.contains_key(&function.name) {
        println!("Function {} found", function.name);
    } else {
        println!("Function {} not found", function.name);
    }

    None
}
