use crate::{execute_sequence, Function, HashMap, Sequence, VarValue};

pub fn while_loop(
    comparator: &Function,
    sequence: &Sequence,
    functions: &mut HashMap<String, Function>,
    variables: &mut HashMap<String, VarValue>,
) {
    let mut i = 0;
    while true {
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
