#[allow(unused_imports)]
use super::{State, SEQUENCER_POS};
use raylib::input::key_from_i32;
use raylib::prelude::*;

pub struct TextInput {
    pub pos: Vector2,
    pub size: Vector2,
    pub bg_color: Color,
    pub text_color: Color,

    pub active: bool,
    pub index: usize,
}

impl TextInput {
    fn is_mouse_in_box(&self, mouse_position: Vector2) -> bool {
        mouse_position.x > self.pos.x
            && mouse_position.x < self.pos.x + self.size.x
            && mouse_position.y > self.pos.y
            && mouse_position.y < self.pos.y + self.size.y
    }

    fn get_input(&mut self, text: &mut String) {
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
                    text.pop();
                }
                KeyboardKey::KEY_ENTER => {
                    self.active = false;
                }
                _ => {
                    let c = key as u8 as char;
                    text.push(c);
                }
            },
            None => {}
        }
    }

    pub fn draw(&mut self, text: &mut String, d: &mut RaylibDrawHandle, state: &mut State) {
        if self.is_mouse_in_box(d.get_mouse_position()) {
            if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                self.active = true;
                state.selected = Some(self.index);
            }
        }
        let mut tmp_bg_color = self.bg_color;
        if state.selected == Some(self.index) {
            self.get_input(text);
            tmp_bg_color = Color::new(255, 255, 255, 255);
        }
        d.draw_rectangle_rounded(
            Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y),
            0.2,
            0,
            tmp_bg_color,
        );
        d.draw_text(
            &text,
            self.pos.x as i32 + 5,
            self.pos.y as i32 + 5,
            20,
            self.text_color,
        );
    }
}

impl TextInput {
    pub fn new(
        pos: Vector2,
        size: Vector2,
        bg_color: Color,
        text_color: Color,
        index: usize,
    ) -> TextInput {
        TextInput {
            pos,
            size,
            bg_color,
            text_color,
            active: false,
            index,
        }
    }
}
