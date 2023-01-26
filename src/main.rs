mod ast;
mod control_flows;
mod functions;
mod native_functions;
mod variables;

use ast::{AstNode, ControlFlow, Function, Sequence, Value};
use control_flows::while_loop;
use functions::{declare_function, execute_function_call};
use native_functions::{execute_native_function, is_native_function};
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
                            name: "println".to_string(),
                            args: vec![
                                Value::Lambda("i: ".to_string()),
                                Value::Variable("i".to_string()),
                            ],
                            body: None,
                        }),
                    ],
                },
            }),
            AstNode::FunctionCall(Function {
                name: "println".to_string(),
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
            match execute_function_call(function, functions, variables) {
                Ok(_) => {}
                Err(_) => {
                    todo!("Function call failed");
                }
            }
        }
        AstNode::FunctionDeclaration(function) => {
            declare_function(function.clone(), functions);
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
