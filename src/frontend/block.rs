use raylib::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub position: Vector2,
    pub size: Vector2,
    pub color: Color,
}

impl Block {
    pub fn new(position: Vector2, size: Vector2, color: Color) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    // translate
    pub fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    pub fn is_point_colliding(&self, point: Vector2) -> bool {
        self.get_rectangle().check_collision_point_rec(point)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rounded(self.get_rectangle(), 0.2, 2, self.color);
        d.draw_rectangle_rounded_lines(self.get_rectangle(), 0.2, 2, 2, self.color);
    }

    pub fn get_rectangle(&self) -> Rectangle {
        Rectangle::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}
