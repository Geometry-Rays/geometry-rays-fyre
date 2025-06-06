use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    velocity_y: &Cell<f32>,
    gravity: f32,
    on_ground: &mut bool,
    jump_force: f32
) {
    if velocity_y.get() < 20.0
    && velocity_y.get() > -20.0 {
        velocity_y.set(velocity_y.get() + gravity);
    }

    if (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && *on_ground {
        velocity_y.set(velocity_y.get() - jump_force);
        *on_ground = false;
    }
}