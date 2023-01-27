use super::{Block, State};
use raylib::prelude::*;

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Block>,
    pub block_index: usize,
    //pub name_input: TextInput,
}

impl FunctionCall {
    pub fn new(state: &mut State) -> Self {
        state.n_blocks += 1;
        state.last_index += 1;
        Self {
            name: "print".to_string(),
            args: vec![],
            block_index: state.n_blocks - 1,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        d.draw_rectangle(0, 0, 100, 100, Color::RED);
        //self.name_input.draw(&mut self.name, d, state);
    }
}
