use raylib::prelude::*;
mod button;
mod function;
mod text_input;
mod variable;

use button::Button;
use function::FunctionCall;
use text_input::TextInput;
use variable::VariableAssignment;

const SEQUENCER_POS: (f32, f32) = (150., 40.);

pub fn draw_sequencer(d: &mut RaylibDrawHandle) {
    d.draw_rectangle_rounded(
        Rectangle::new(SEQUENCER_POS.0, SEQUENCER_POS.1, 300.0, 400.0),
        0.2,
        0,
        Color::GRAY,
    );
}

pub enum Block {
    VariableAssignment,
    FunctionCall,
}

pub struct State {
    pub last_index: usize,
    pub selected: Option<usize>,
    pub n_blocks: usize,
}

pub fn main_loop() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .msaa_4x()
        .vsync()
        //.transparent()
        .undecorated()
        .build();

    let mut blocks = vec![Block::VariableAssignment, Block::FunctionCall];

    let buttons = vec![
        Button::new(
            Vector2::new(10., 10.),
            Vector2::new(100., 40.),
            "Exit".to_string(),
        ),
        Button::new(
            Vector2::new(10., 60.),
            Vector2::new(100., 40.),
            "Run".to_string(),
        ),
        Button::new(
            Vector2::new(10., 110.),
            Vector2::new(100., 40.),
            "Add".to_string(),
        ),
    ];

    loop {
        let mut d = rl.begin_drawing(&thread);
        if d.window_should_close() || d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        d.clear_background(Color::WHITE);
        draw_sequencer(&mut d);
        for block in blocks.iter() {
            match block {
                Block::VariableAssignment => {}
                Block::FunctionCall => {}
            }
        }
        for button in buttons.iter() {
            button.draw(&mut d);
        }
    }
}
