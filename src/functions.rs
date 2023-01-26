use crate::{execute_native_function, is_native_function, Function, HashMap, VarValue};

pub fn declare_function(function: Function, functions: &mut HashMap<String, Function>) {
    functions.insert(function.name.clone(), function);
}

pub fn execute_function_call(
    function: &Function,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) -> Result<Option<VarValue>, ()> {
    if is_native_function(&function.name) {
        return execute_native_function(function, functions, variables);
    }
    if !functions.contains_key(&function.name) {
        println!("Function {} not found", function.name);
        return Err(());
    }
    Ok(None)
}
