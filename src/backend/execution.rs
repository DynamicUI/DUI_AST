#[allow(unused_imports)]
use crate::backend::{
    ast::{AstNode, ControlFlow, FunctionsMap, Sequence, VariablesMap},
    control_flows::while_loop,
    functions::{declare_function, execute_function_call},
    variables::assign_variable,
};

pub fn execute_ast_node(
    node: &AstNode,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) {
    match node {
        AstNode::VariableAssignment { name, value } => {
            assign_variable(name, value, functions, variables);
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

pub fn execute_sequence(
    sequence: &Sequence,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) {
    for node in sequence.sequence.iter() {
        execute_ast_node(node, functions, variables);
    }
}
