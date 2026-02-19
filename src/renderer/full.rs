use orx_concurrent_vec::ConcurrentElement;

use crate::capsule::{
    Capsule,
    obj::{BoxedCapsuleObject, CapsuleObject},
};

pub fn render_capsule(capsule: &Capsule) {
    fn render_object(object: &ConcurrentElement<BoxedCapsuleObject>) {
        object.map(|o| o.render());

        // render children
        let base = object.map(|o| o.base());

        for obj in base.children.iter() {
            render_object(obj);
        }
    }

    let base = capsule.view.base();

    for obj in base.children.iter() {
        render_object(obj);
    }
}
