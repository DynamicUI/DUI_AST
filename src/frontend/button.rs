use super::Block;
use raylib::prelude::*;

pub struct Button {
    pub block: Block,
    pub text: String,
}

impl Button {
    pub fn new(block: Block, text: String) -> Self {
        Self { block, text }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.block.draw(d);
        d.draw_text(
            &self.text,
            self.block.position.x as i32,
            self.block.position.y as i32,
            20,
            Color::BLACK,
        );
    }

    pub fn is_clicked(&self, d: &RaylibDrawHandle) -> bool {
        d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
            && self.block.is_point_colliding(d.get_mouse_position())
    }
}
