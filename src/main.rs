#[allow(unused_imports)]
mod ast;
mod control_flows;
mod functions;
mod native_functions;
mod variables;

use ast::{
    AstNode, ControlFlow, FunctionCall, FunctionSignature, FunctionsMap, Sequence, ValueGetter,
    VariablesMap,
};
use control_flows::while_loop;
use functions::{declare_function, execute_function_call};
use native_functions::{execute_native_function, is_native_function};
use std::collections::HashMap;
use variables::{get_var_value, Value, VarType, Variable};

fn main() {
    let mut functions: FunctionsMap = FunctionsMap(HashMap::new());
    let mut variables: VariablesMap = VariablesMap(HashMap::new());
    let sequence = Sequence {
        sequence: vec![
            AstNode::VariableAssignment {
                name: "i".to_string(),
                value: ValueGetter::Lambda("0".to_string()),
            },
            AstNode::FunctionDeclaration(FunctionSignature {
                name: "printI".to_string(),
                args: HashMap::from([("i_ref".to_string(), Some(Variable::from(0)))]),
                body: Sequence {
                    sequence: vec![
                        AstNode::VariableAssignment {
                            name: "string".to_string(),
                            value: ValueGetter::FunctionCall(FunctionCall {
                                name: "add".to_string(),
                                args: vec![
                                    ValueGetter::Lambda("i is equal to ".to_string()),
                                    ValueGetter::Variable("i_ref".to_string()),
                                ],
                            }),
                        },
                        AstNode::FunctionCall(FunctionCall {
                            name: "println".to_string(),
                            args: vec![ValueGetter::Variable("string".to_string())],
                        }),
                    ],
                },
            }),
            AstNode::ControlFlow(ControlFlow::WhileLoop {
                fn_condition: FunctionCall {
                    name: "less_than".to_string(),
                    args: vec![
                        ValueGetter::Variable("i".to_string()),
                        ValueGetter::Lambda("10".to_string()),
                    ],
                },
                body: Sequence {
                    sequence: vec![
                        AstNode::VariableAssignment {
                            name: "i".to_string(),
                            value: ValueGetter::FunctionCall(FunctionCall {
                                name: "add".to_string(),
                                args: vec![
                                    ValueGetter::Variable("i".to_string()),
                                    ValueGetter::Lambda("1".to_string()),
                                ],
                            }),
                        },
                        AstNode::FunctionCall(FunctionCall {
                            name: "printI".to_string(),
                            args: vec![ValueGetter::Variable("i".to_string())],
                        }),
                    ],
                },
            }),
            AstNode::FunctionCall(FunctionCall {
                name: "println".to_string(),
                args: vec![ValueGetter::Lambda("Fin de la boucle !".to_string())],
            }),
        ],
    };

    execute_sequence(&sequence, &mut functions, &mut variables);
}

fn execute_ast_node(node: &AstNode, functions: &mut FunctionsMap, variables: &mut VariablesMap) {
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
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) {
    for node in sequence.sequence.iter() {
        execute_ast_node(node, functions, variables);
    }
}
