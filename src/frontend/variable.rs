use super::{text_input::TextInput, State, SEQUENCER_POS};
use raylib::prelude::*;

pub struct VariableAssignment {
    pub name: String,
    pub value: String,
    pub name_input: TextInput,
    pub value_input: TextInput,
    pub block_index: usize,
    //pos: (f32, f32),
    //size: (f32, f32),
    //color: Color,
    //active: bool,
}

impl VariableAssignment {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        let gray_level = 80;
        d.draw_rectangle_rounded(
            Rectangle::new(SEQUENCER_POS.0 + 10., SEQUENCER_POS.1 + 10., 280.0, 130.0),
            0.2,
            0,
            Color::new(gray_level, gray_level, gray_level, 255),
        );
        self.name_input.draw(&mut self.name, d, state);
        self.value_input.draw(&mut self.value, d, state);
    }
}

impl VariableAssignment {
    pub fn new(state: &mut State) -> Self {
        state.last_index += 2;
        state.n_blocks += 1;
        Self {
            name: String::new(),
            value: String::new(),
            name_input: TextInput::new(
                Vector2::new(SEQUENCER_POS.0 + 20., SEQUENCER_POS.1 + 40.),
                Vector2::new(260., 40.),
                Color::new(200, 200, 200, 255),
                Color::BLACK,
                state.last_index - 2,
            ),

            value_input: TextInput::new(
                Vector2::new(SEQUENCER_POS.0 + 20., SEQUENCER_POS.1 + 90.),
                Vector2::new(260., 40.),
                Color::new(200, 200, 200, 255),
                Color::BLACK,
                state.last_index - 1,
            ),
            block_index: state.n_blocks - 1,
        }
    }
}
