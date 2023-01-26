use crate::{execute_sequence, Function, HashMap, Sequence, VarName, VarValue};

pub fn while_loop(
    comparator: &Function,
    sequence: &Sequence,
    variables: &mut HashMap<VarName, VarValue>,
) {
    let mut i = 0;
    while true {
        execute_sequence(&sequence, variables);
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
