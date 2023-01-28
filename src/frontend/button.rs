use super::Block;
use raylib::prelude::*;


pub struct Button {
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,
    pub text: String,
    is_down: bool,
}

impl Button {
    pub fn new(position: Vector2, size: Vector2, color: Color, text: String) -> Self {
        return Self { position, size, color, text, is_down: false };
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
            && self.is_point_colliding(rl.get_mouse_position()) {
            self.is_down = true;
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            self.is_down = false;
        }
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        return rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
            && self.is_point_colliding(rl.get_mouse_position());
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rounded(self.get_rectangle(), 0.2, 2, self.color);
        if self.is_down {
            d.draw_rectangle_rounded(self.get_rectangle(), 0.2, 2, Color::BLACK.fade(0.3));
        }
        d.draw_rectangle_rounded_lines(self.get_rectangle(), 0.25, 2, 2, Color::BLACK);

        d.draw_text(
            &self.text,
            self.position.x as i32,
            self.position.y as i32,
            20,
            Color::BLACK,
        );
    }

    fn get_rectangle(&self) -> Rectangle {
        return Rectangle::new(self.position.x, self.position.y, self.size.x, self.size.y);
    }

    pub fn is_point_colliding(&self, point: Vector2) -> bool {
        self.get_rectangle().check_collision_point_rec(point)
    }
}
