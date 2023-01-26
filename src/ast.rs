use crate::{Condition, VarName, VarType, VarValue};
use std::collections::LinkedList;

pub enum AstResult {
    VarValue(VarValue),
    NoVarFound,
    None,
}

pub enum AST {
    Var(Var),
    Block(Block),
    Func(String),
    Op(Op),
}

pub enum Var {
    Assign(VarName, VarValue),
    Lambda(VarValue),
    Ref(VarName),
}

pub enum Block {
    While(Condition, LinkedList<AST>),
}

pub enum Op {
    Add(Var, Var),
    LessThan,
}
