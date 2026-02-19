use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
};

use crate::capsule::{Capsule, obj::iter_all_objects};

pub fn update_events(capsule: &mut Capsule) {
    if is_mouse_button_pressed(MouseButton::Left) {
        // onclick event
        // check what we clicked on
        let mouse_position = Vec2::from(mouse_position());
        let mut callbacks = vec![];

        iter_all_objects(capsule, |o| {
            let bb = o.map(|o| o.bounding_box());

            if bb.contains(mouse_position) {
                o.map(|o| {
                    for e in o
                        .base()
                        .events
                        .iter()
                        .filter(|e| e.map(|u| u.name == "onclick"))
                    {
                        callbacks.push(e.map(|u| u.callback.clone()));
                    }
                });
            }
        });

        let lua = &mut capsule.lua;

        for cb in callbacks {
            lua.get_function(&cb).unwrap().call::<()>(()).unwrap();
        }
    }
}
