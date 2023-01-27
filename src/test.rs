fn test_execution() {
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
