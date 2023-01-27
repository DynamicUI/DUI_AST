use super::{Block, State};
use raylib::prelude::*;

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Block>,
}

impl FunctionCall {
    pub fn new(name: String, args: Vec<Block>, state: &mut State) -> Self {
        Self { name, args }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, state: &mut State) {
        d.draw_rectangle(0, 0, 100, 100, Color::RED);
    }
}
