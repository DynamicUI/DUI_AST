mod ast;
mod control_flows;
mod native_functions;
mod variables;

use ast::{AstNode, Function, Sequence, Value};

fn main() {
    let sequence = vec![
        AstNode::VariableAssignment {
            name: "i".to_string(),
            value: Value::Lambda("0".to_string()),
        },
        AstNode::ControlFlow {
            fn_condition: Function {
                name: "less_than".to_string(),
                parameters: vec!["i".to_string(), "10".to_string()],
                body: None,
            },
            body: vec![AstNode::VariableAssignment {
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
        AstNode::FunctionCall(Function {
            name: "print".to_string(),
            parameters: vec!["i".to_string()],
            body: None,
        }),
    ];
}
