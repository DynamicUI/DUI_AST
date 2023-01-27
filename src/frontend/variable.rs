use super::{text_input::TextInput, Block, State};
use raylib::prelude::*;

pub struct VariableAssignment {
    pub name: String,
    pub name_input: TextInput,
    pub block: Block,
}

const DEFAULT_SIZE: Vector2 = Vector2::new(120., 80.);

impl VariableAssignment {
    pub fn new(position: Vector2, input_index: usize) -> Self {
        Self {
            name: String::new(),
            name_input: TextInput::new(
                Vector2::new(position.x + 10., position.y + 20.),
                input_index,
            ),
            block: Block::new(position, DEFAULT_SIZE, Color::LIGHTGRAY),
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        self.block.draw(d);
        self.name_input.draw(d, state);
    }
}
