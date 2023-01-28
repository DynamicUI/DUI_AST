use raylib::prelude::*;

mod action_block;
mod block;
mod button;
mod function;
mod text_input;
mod variable;

use action_block::ActionBlock;
use block::Block;
use button::Button;
use function::FunctionCall;
use variable::VariableAssignment;

pub struct State {
    pub last_index: usize,
    pub selected: Option<usize>,
}

fn init() -> (
    (RaylibHandle, RaylibThread),
    Vec<ActionBlock>,
    Vec<Button>,
    bool,
    State,
) {
    let mut state = State {
        last_index: 0,
        selected: None,
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
            Color::LIGHTGRAY,
            "Exit".to_string(),
        ),
        Button::new(
            Vector2::new(10., 60.),
            Vector2::new(100., 40.),
            Color::LIGHTGRAY,
            "Run".to_string(),
        ),
        Button::new(
            Vector2::new(10., 110.),
            Vector2::new(100., 40.),
            Color::LIGHTGRAY,
            "Add".to_string(),
        ),
    ];

    return (
        raylib::init()
            .size(1280, 700)
            .msaa_4x()
            .vsync()
            .resizable()
            //.transparent()
            //.undecorated()
            .build(),
        blocks,
        buttons,
        false,
        state,
    );
}

pub fn main_loop() {
    let ((mut rl, thread),
        mut blocks,
        mut buttons,
        mut should_exit,
        mut state) = init();

    while !rl.window_should_close() && !should_exit {
        // Update
        handle_focus(&mut rl, &mut state);
        blocks.iter_mut().for_each(|b| b.update(&mut rl, &mut state));
        buttons.iter_mut().for_each(|b| b.update(&mut rl));
        handle_buttons(&buttons, &mut rl, &mut state, &mut blocks);
        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        blocks.iter_mut().for_each(|b| b.draw(&mut d, &mut state));
        buttons.iter().for_each(|b| b.draw(&mut d));
    }
}


pub fn handle_buttons(buttons: &Vec<Button>,
                      rl: &mut RaylibHandle,
                      state: &mut State,
                      blocks: &mut Vec<ActionBlock>)
{
    for button in buttons {
        if button.is_clicked(rl) {
            match button.text.as_str() {
                "Exit" => should_exit = true,
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

fn handle_focus(rl: &mut RaylibHandle,
                state: &mut State)
{
    if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
        state.selected = match state.selected {
            Some(i) => Some((i + 1) % state.last_index),
            None => Some(0),
        };
    }
}
