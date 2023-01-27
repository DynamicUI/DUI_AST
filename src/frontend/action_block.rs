use super::{FunctionCall, State, VariableAssignment};
use raylib::prelude::*;

pub trait ActionBlockTrait {
    fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State);
}

pub enum ActionBlock {
    VariableAssignment(VariableAssignment),
    FunctionCall(FunctionCall),
}

macro_rules! block_match {
    ($block:ident, $d:ident, $state:ident, $($type:ident),+) => {
        match $block {
            $(ActionBlock::$type(va) => va.draw($d, $state)),+
        }
    };
}

impl ActionBlock {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        block_match!(self, d, state, VariableAssignment, FunctionCall);
    }
}
