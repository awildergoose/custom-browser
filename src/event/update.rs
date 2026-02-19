use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
};

use crate::capsule::{
    Capsule,
    obj::{ArcLock, iter_all_objects},
};

pub fn update_events(capsule: &ArcLock<Capsule>) {
    let is_m1_pressed = is_mouse_button_pressed(MouseButton::Left);
    let is_m2_pressed = is_mouse_button_pressed(MouseButton::Right);
    let is_m3_pressed = is_mouse_button_pressed(MouseButton::Middle);

    if is_m1_pressed || is_m2_pressed || is_m3_pressed {
        let mouse_position = Vec2::from(mouse_position());
        let mut callbacks = vec![];

        let capsule_read = capsule.read();
        iter_all_objects(&capsule_read, |o| {
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

        let mut lua = capsule_read.lua.write();

        let mut buttons = vec![];

        if is_m1_pressed {
            buttons.push(1);
        }
        if is_m2_pressed {
            buttons.push(2);
        }
        if is_m3_pressed {
            buttons.push(3);
        }

        for cb in callbacks {
            for btn in &buttons {
                if let Err(e) = lua.get_function(&cb).unwrap().call::<()>(*btn) {
                    log::error!("Lua error: {e}");
                }
            }
        }

        drop(lua);
        drop(capsule_read);
    }
}
