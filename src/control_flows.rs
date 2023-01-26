use crate::{execute_function_call, execute_sequence, Function, HashMap, Sequence, VarValue};

pub fn while_loop(
    comparator: &Function,
    sequence: &Sequence,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) {
    let mut i = 0;
    loop {
        match execute_function_call(comparator, functions, variables) {
            Ok(Some(value)) => {
                if value.value == "true" {
                    execute_sequence(sequence, functions, variables);
                    i += 1;
                } else {
                    break;
                }
            }
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
