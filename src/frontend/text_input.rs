#[allow(unused_imports)]
use super::{
    ActionBlock, ActionBlockTrait, Block, Button, FunctionCall, State, VariableAssignment,
};
use raylib::input::key_from_i32;
use raylib::prelude::*;

pub struct TextInput {
    pub block: Block,
    pub text: String,
    pub index: usize,
}
const DEFAULT_HEIGHT: f32 = 30.;

impl TextInput {
    pub fn new(position: Vector2, index: usize) -> Self {
        return Self {
            block: Block::new(position, Vector2::new(60., DEFAULT_HEIGHT), Color::WHITE),
            text: String::new(),
            index,
        };
    }

    fn get_input(&mut self) {
        /* Code source (je n'arrive pas a deferencer le baille) */
        let key = unsafe { ffi::GetKeyPressed() };
        if key <= 0 {
            return;
        }
        let key = key_from_i32(key);
        /* Fin */
        match key {
            Some(key) => match key {
                KeyboardKey::KEY_BACKSPACE => {
                    self.text.pop();
                }
                KeyboardKey::KEY_ENTER => {
                    //self.active = false;
                }
                _ => {
                    let c = key as u8 as char;
                    self.text.push(c);
                }
            },
            None => {}
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &State) {
        let mut tmp_bg_color = Color::LIGHTGRAY;
        if state.selected == Some(self.index) {
            self.get_input();
            tmp_bg_color = Color::WHITE;
        }
        d.draw_rectangle_rounded(self.block.get_rectangle(), 0.2, 0, tmp_bg_color);
        d.draw_text(
            &self.text,
            self.block.position.x as i32 + 5,
            self.block.position.y as i32 + 5,
            20,
            Color::BLACK,
        );
    }
}
