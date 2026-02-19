use macroquad::shapes::draw_rectangle;

use crate::capsule::{Capsule, obj::iter_all_objects};

pub fn render_capsule(capsule: &Capsule) {
    iter_all_objects(capsule, |o| {
        o.map(|o| {
            let binding = o.base();
            let style = binding.style.read();
            let computed = binding.computed_style.read();

            if let Some(color) = style.background_color {
                draw_rectangle(
                    computed.x,
                    computed.y,
                    computed.width,
                    computed.height,
                    color.as_macroquad(),
                );
            }

            drop(style);
            drop(computed);
            drop(binding);

            o.render();
        });
    });
}
