#[allow(unused_imports)]
use crate::{
    backend::execution::execute_sequence, execute_function_call, FunctionCall, FunctionsMap,
    Sequence, Value, VariablesMap,
};

pub fn while_loop(
    comparator: &FunctionCall,
    sequence: &Sequence,
    functions: &mut FunctionsMap,
    variables: &mut VariablesMap,
) {
    let mut i = 0;
    loop {
        match execute_function_call(comparator, functions, variables) {
            Ok(Some(variable)) => match variable.value {
                Value::Bool(value) => {
                    if value {
                        execute_sequence(sequence, functions, variables);
                        i += 1;
                    } else {
                        break;
                    }
                }
                _ => todo!("while loop condition must return a boolean"),
            },
            Ok(None) => {
                todo!("While loop comparator must return a value");
            }
            Err(_) => {
                todo!("Function call failed");
            }
        }

        execute_sequence(sequence, functions, variables);

        if i >= 100 {
            println!(
                "Infinite loop detected ! (more than {}, see in parameters to change the max)",
                i
            );
            break;
        }
        i += 1;
    }
}
