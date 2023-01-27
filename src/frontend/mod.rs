use raylib::input::key_from_i32;
use raylib::prelude::*;
use std::ffi::CStr;

const SEQUENCER_POS: (f32, f32) = (150., 40.);

pub fn draw_sequencer(d: &mut RaylibDrawHandle) {
    d.draw_rectangle_rounded(
        Rectangle::new(SEQUENCER_POS.0, SEQUENCER_POS.1, 300.0, 400.0),
        0.2,
        0,
        Color::GRAY,
    );
    //d.draw_rectangle_rounded(150, 40, 300, 400, Color::GRAY);
}

struct TextInput {
    text: String,
    pos: (f32, f32),
    size: (f32, f32),
    color: Color,
    active: bool,
}
impl TextInput {
    fn get_input(&mut self, d: &mut RaylibDrawHandle) {
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
                    self.active = false;
                }
                _ => {
                    let c = key as u8 as char;
                    self.text.push(c);
                }
            },
            None => {}
        }
    }
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.get_input(d);
        d.draw_rectangle_rounded(
            Rectangle::new(self.pos.0, self.pos.1, self.size.0, self.size.1),
            0.2,
            0,
            self.color,
        );
        d.draw_text(
            &self.text,
            self.pos.0 as i32 + 5,
            self.pos.1 as i32 + 5,
            20,
            Color::WHITE,
        );
    }
}
impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: String::new(),
            pos: (0., 0.),
            size: (300., 100.),
            color: Color::GRAY,
            active: false,
        }
    }
}

pub fn draw_variable(d: &mut RaylibDrawHandle) {
    let gray_level = 80;
    d.draw_rectangle_rounded(
        Rectangle::new(SEQUENCER_POS.0 + 10., SEQUENCER_POS.1 + 10., 280.0, 100.0),
        0.2,
        0,
        Color::new(gray_level, gray_level, gray_level, 255),
    );
    let mut text_input = TextInput::default();
    text_input.draw(d);
}

pub fn main_loop() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .msaa_4x()
        .vsync()
        //.transparent()
        .undecorated()
        .build();

    loop {
        let mut d = rl.begin_drawing(&thread);
        if d.window_should_close() || d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        d.clear_background(Color::WHITE);
        draw_sequencer(&mut d);
        draw_variable(&mut d);
    }
}
