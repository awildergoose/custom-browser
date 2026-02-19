use macroquad::input::{MouseButton, is_mouse_button_pressed, mouse_position};

use crate::capsule::Capsule;

pub fn update_events(capsule: &mut Capsule) {
    if is_mouse_button_pressed(MouseButton::Left) {
        // onclick event
        // check what we clicked on
        let mouse_position = mouse_position();
    }
}
