mod backend;
mod frontend;

#[allow(unused_imports)]
use backend::{
    ast::{
        AstNode, ControlFlow, FunctionCall, FunctionSignature, FunctionsMap, Sequence, ValueGetter,
        VariablesMap,
    },
    execution::execute_sequence,
    functions::{declare_function, execute_function_call},
    native_functions::{execute_native_function, is_native_function},
    variables::{assign_variable, get_var_value, Value, VarType, Variable},
};
use std::collections::HashMap;

fn main() {
    frontend::main_loop();

    // for backend
    // let mut functions: FunctionsMap = FunctionsMap(HashMap::new());
    // let mut variables: VariablesMap = VariablesMap(HashMap::new());
}
