use super::{FunctionCall, State, VariableAssignment};
use raylib::prelude::*;

pub enum ActionBlock {
    VariableAssignment(VariableAssignment),
    FunctionCall(FunctionCall),
}

/// Macro to match every block type and draw it
macro_rules! block_match {
    ($block:ident, $d:ident, $state:ident, $($type:ident),+) => {
        match $block {
            $(ActionBlock::$type(va) => va.draw($d, $state)),+
        }
    };
}

macro_rules! block_match_update {
    ($block:ident, $rl:ident, $state:ident, $($type:ident),+) => {
        match $block {
            $(ActionBlock::$type(va) => va.update($rl, $state)),+
        }
    };
}

/// Match every block type and draw it
impl ActionBlock {
    pub fn update(&mut self, rl: &mut RaylibHandle, state: &mut State) {
        block_match_update!(self, rl, state, VariableAssignment, FunctionCall);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        block_match!(self, d, state, VariableAssignment, FunctionCall);
    }
}
