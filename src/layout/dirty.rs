use crate::{
    capsule::{
        Capsule,
        obj::{ArcLock, iter_all_objects},
    },
    layout::computer::compute_layout,
};

pub fn update_layout(capsule: &ArcLock<Capsule>) {
    let mut is_dirty = false;

    {
        iter_all_objects(&capsule.read(), |e| {
            if e.map(|e| e.is_dirty()) {
                is_dirty = true;

                e.map(|e| {
                    *e.base().dirty.write() = false;
                });
            }
        });
    }

    if is_dirty {
        compute_layout(&mut capsule.write());
    }
}
