use raylib::prelude::*;

pub struct Button {
    pub position: Vector2,
    pub size: Vector2,
    pub text: String,
    pub bg_color: Color,
    pub text_color: Color,
}

impl Button {
    pub fn new(position: Vector2, size: Vector2, text: String) -> Self {
        Self {
            position,
            size,
            text,
            bg_color: Color::GRAY,
            text_color: Color::BLACK,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            self.size.x as i32,
            self.size.y as i32,
            self.bg_color,
        );
        d.draw_text(
            &self.text,
            self.position.x as i32,
            self.position.y as i32,
            20,
            self.text_color,
        );
    }

    pub fn is_clicked(&self, d: &RaylibDrawHandle) -> bool {
        let mouse_position = d.get_mouse_position();
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return mouse_position.x > self.position.x
                && mouse_position.x < self.position.x + self.size.x
                && mouse_position.y > self.position.y
                && mouse_position.y < self.position.y + self.size.y;
        }
        false
    }
}
