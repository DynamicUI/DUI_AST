use crate::{Op, Var};

pub struct Condition {
    pub left: Var,  // Cannot be a Var::Assign
    pub right: Var, // Cannot be a Var::Assign
    pub op: Op,
}
