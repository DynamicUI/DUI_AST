use crate::{
    execute_native_function, execute_sequence, is_native_function, Function, HashMap, VarValue,
};

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
        todo!("Function {} not found", function.name);
    }
    let function: Function = functions.get(&function.name).unwrap().clone();
    let mut variables: HashMap<String, VarValue> = HashMap::new(); // = variables.clone();
    todo!("add args to variables");
    match function.body {
        Some(body) => {
            execute_sequence(&body, functions, &mut variables);
        }
        None => {
            todo!("Function {} has no body", function.name);
        }
    }
    Ok(None)
}
