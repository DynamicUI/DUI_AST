mod ast;
mod control_flows;
mod native_functions;
mod variables;

use ast::{AstNode, ControlFlow, Function, Sequence, Value};
use control_flows::while_loop;
use std::collections::HashMap;
use variables::{VarType, VarValue};

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
                    parameters: vec!["i".to_string(), "10".to_string()],
                    body: None,
                },
                body: Sequence {
                    sequence: vec![AstNode::VariableAssignment {
                        name: "i".to_string(),
                        value: Value::FunctionCall {
                            name: "add".to_string(),
                            args: vec![
                                Value::Variable("i".to_string()),
                                Value::Lambda("1".to_string()),
                            ],
                        },
                    }],
                },
            }),
            AstNode::FunctionCall(Function {
                name: "print".to_string(),
                parameters: vec!["i".to_string()],
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
            variables::assign_variable(name, value, variables);
        }
        AstNode::ControlFlow(control_flow) => match control_flow {
            ControlFlow::WhileLoop { fn_condition, body } => {
                while_loop(fn_condition, body, functions, variables);
            }
        },
        AstNode::FunctionCall(function) => {
            //native_functions::execute_function(function);
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
    name: &str,
    args: &Vec<Value>,
    variables: &mut HashMap<String, VarValue>,
) -> Option<VarValue> {
    // first check if it's a native function
    None
}
