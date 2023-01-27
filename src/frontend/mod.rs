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
    VariableAssignment(VariableAssignment),
    FunctionCall(FunctionCall),
}

macro_rules! block_match {
    ($block:ident, $d:ident, $state:ident, $($type:ident),*) => {
        match $block {
            $(Block::$type(va) => va.draw($d, $state)),*
        }
    };
}

impl Block {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, state: &mut State) {
        block_match!(self, d, state, VariableAssignment, FunctionCall);
    }
}

/*
pub enum ValueGetter {
    Variable(String),
    Lambda(String),
    FunctionCall(FunctionCall),
}
*/

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

    let mut state = State {
        last_index: 0,
        selected: None,
        n_blocks: 0,
    };

    let mut blocks = vec![
        Block::VariableAssignment(VariableAssignment::new(&mut state)),
        Block::FunctionCall(FunctionCall::new(&mut state)),
    ];

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

    'main: loop {
        let mut d = rl.begin_drawing(&thread);
        if d.window_should_close() || d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        d.clear_background(Color::WHITE);
        draw_sequencer(&mut d);

        // focus handling
        if d.is_key_pressed(KeyboardKey::KEY_TAB) {
            state.selected = match state.selected {
                Some(i) => Some((i + 1) % state.last_index),
                None => Some(0),
            };
        }

        for block in &mut blocks {
            block.draw(&mut d, &mut state);
        }
        for button in buttons.iter() {
            button.draw(&mut d);
            if button.is_clicked(&mut d) {
                match button.text.as_str() {
                    "Exit" => break 'main,
                    "Run" => {
                        for block in &blocks {
                            match block {
                                Block::VariableAssignment(va) => {
                                    println!("{} = {}", va.name, va.value)
                                }
                                Block::FunctionCall(fc) => {
                                    println!("{}({})", fc.name, fc.args.len())
                                }
                            }
                        }
                    }
                    "Add" => {}
                    _ => {}
                }
            }
        }
    }
}
