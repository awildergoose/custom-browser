use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
};

use crate::capsule::{
    Capsule,
    obj::{ArcLock, iter_all_objects},
};

pub fn update_events(capsule: &ArcLock<Capsule>) {
    if is_mouse_button_pressed(MouseButton::Left) {
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

        for cb in callbacks {
            lua.get_function(&cb).unwrap().call::<()>(()).unwrap();
        }

        drop(lua);
        drop(capsule_read);
    }
}
