use raylib::prelude::*;
mod action_block;
mod block;
mod button;
mod function;
mod text_input;
mod variable;

use action_block::{ActionBlock, ActionBlockTrait};
use block::Block;
use button::Button;
use function::FunctionCall;
use variable::VariableAssignment;

fn init() -> (
    (RaylibHandle, RaylibThread),
    Vec<ActionBlock>,
    Vec<Button>,
    State,
) {
    let mut state = State {
        last_index: 0,
        selected: None,
        n_blocks: 0,
        should_exit: false,
    };

    let blocks = vec![
        ActionBlock::VariableAssignment(VariableAssignment::new(
            Vector2::new(300., 300.),
            state.last_index,
        )),
        ActionBlock::VariableAssignment(VariableAssignment::new(
            Vector2::new(300., 200.),
            state.last_index + 1,
        )),
        //ActionBlock::FunctionCall(FunctionCall::new(&mut state)),
    ];
    state.last_index += 2;

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
    return (
        raylib::init()
            .size(640, 480)
            .msaa_4x()
            .vsync()
            //.transparent()
            .undecorated()
            .build(),
        blocks,
        buttons,
        state,
    );
}

pub struct State {
    pub last_index: usize,
    pub selected: Option<usize>,
    pub n_blocks: usize,
    pub should_exit: bool,
}

pub fn handle_button(
    buttons: &Vec<Button>,
    d: &mut RaylibDrawHandle,
    state: &mut State,
    blocks: &mut Vec<ActionBlock>,
) {
    for button in buttons {
        if button.is_clicked(d) {
            match button.text.as_str() {
                "Exit" => state.should_exit = true,
                "Run" => {
                    for block in &mut *blocks {
                        match block {
                            ActionBlock::VariableAssignment(va) => {
                                println!("{} = ", va.name)
                            }
                            ActionBlock::FunctionCall(fc) => {
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

fn handle_focus(d: &mut RaylibDrawHandle, state: &mut State) {
    if d.is_key_pressed(KeyboardKey::KEY_TAB) {
        state.selected = match state.selected {
            Some(i) => Some((i + 1) % state.last_index),
            None => Some(0),
        };
    }
}

pub fn main_loop() {
    let ((mut rl, thread), mut blocks, buttons, mut state) = init();
    while !rl.window_should_close() && !state.should_exit {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        handle_focus(&mut d, &mut state);
        blocks.iter_mut().for_each(|b| b.draw(&mut d, &mut state));
        buttons.iter().for_each(|b| b.draw(&mut d));
        handle_button(&buttons, &mut d, &mut state, &mut blocks);
    }
}
