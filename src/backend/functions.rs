#[allow(unused_imports)]
use crate::{
    execute_native_function, backend::execution::execute_sequence, is_native_function, FunctionCall, FunctionSignature,
    FunctionsMap, HashMap, ValueGetter, Variable, VariablesMap,
};

pub fn declare_function(function: FunctionSignature, functions: &mut FunctionsMap) {
    functions.0.insert(function.name.clone(), function);
}

pub fn execute_function_call(
    function_call: &FunctionCall,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) -> Result<Option<Variable>, ()> {
    if is_native_function(&function_call.name) {
        return execute_native_function(function_call, functions, variables);
    } else if !functions.0.contains_key(&function_call.name) {
        todo!("Function {} not found", function_call.name);
    }

    let mut scope_variables: VariablesMap = VariablesMap(HashMap::new());
    let function: FunctionSignature = functions.0.get(&function_call.name).unwrap().clone();
    if function.args.len() != function_call.args.len() {
        todo!(
            "Function {} called with wrong number of arguments",
            function.name
        );
    }

    for (function_arg, call_arg) in function.args.iter().zip(function_call.args.iter()) {
        let arg_name = function_arg.0;
        let arg_value = function_arg.1;
        match arg_value {
            Some(value) => {
                scope_variables
                    .0
                    .insert(arg_name.to_string(), value.clone());
            }
            None => {
                scope_variables
                    .0
                    .insert(arg_name.to_string(), Variable::from("None"));
            }
        }
        match call_arg {
            ValueGetter::Variable(name) => {
                let value = variables.0.get(name).unwrap().clone();
                scope_variables.0.insert(arg_name.to_string(), value);
            }
            ValueGetter::Lambda(value) => {
                scope_variables
                    .0
                    .insert(arg_name.to_string(), Variable::from(value.clone()));
            }
            ValueGetter::FunctionCall(function_call) => {
                let value = execute_function_call(function_call, functions, variables)?;
                scope_variables
                    .0
                    .insert(arg_name.to_string(), value.unwrap());
            }
        }
    }
    execute_sequence(&function.body, functions, &mut scope_variables);
    Ok(None)
}
