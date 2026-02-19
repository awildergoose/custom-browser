use crate::capsule::{Capsule, obj::iter_all_objects};

pub fn render_capsule(capsule: &Capsule) {
    iter_all_objects(capsule, |o| o.map(|o| o.render()));
}
