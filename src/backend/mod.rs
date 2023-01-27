pub mod ast;
pub mod control_flows;
pub mod execution;
pub mod functions;
pub mod native_functions;
pub mod variables;

use self::{
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
