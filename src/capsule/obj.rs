use std::{fmt::Debug, sync::Arc};

use macroquad::math::Rect;
use orx_concurrent_vec::{ConcurrentElement, ConcurrentVec};
use parking_lot::RwLock;

use crate::{
    capsule::objs::{script::CSScript, view::CapsuleView},
    event::CapsuleObjectEvent,
    layout::{computed::ComputedStyling, styling::Styling},
    lua::engine::LuaEngine,
};

pub type BoxedCapsuleObject = Box<dyn CapsuleObject + Sync + Send>;
pub type CapsuleObjectChildren = Arc<ConcurrentVec<BoxedCapsuleObject>>;
pub type CapsuleObjectEvents = Arc<ConcurrentVec<CapsuleObjectEvent>>;

pub trait CapsuleObject: Debug {
    fn base(&self) -> Arc<CapsuleObjectBase>;
    fn render(&self);
    fn bounding_box(&self) -> Rect {
        let base = self.base();
        let computed = base.computed_style.read();

        Rect::new(computed.x, computed.y, computed.width, computed.height)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CapsuleObjectBase {
    pub children: CapsuleObjectChildren,
    pub events: CapsuleObjectEvents,
    pub style: Arc<Styling>,
    pub computed_style: Arc<RwLock<ComputedStyling>>,
}

impl CapsuleObjectBase {
    pub fn new(
        children: CapsuleObjectChildren,
        style: Arc<Styling>,
        events: CapsuleObjectEvents,
    ) -> Arc<Self> {
        Arc::new(Self {
            children,
            style,
            events,
            computed_style: Arc::default(),
        })
    }
}

#[derive(Debug, Default)]
pub struct CapsuleMeta {
    pub title: String,
    pub scripts: Vec<CSScript>,
}

#[derive(Debug, Default)]
pub struct Capsule {
    pub meta: CapsuleMeta,
    pub view: CapsuleView,
    pub lua: LuaEngine,
}

impl Capsule {
    pub fn run_scripts(&mut self) {
        self.lua = LuaEngine::default();

        let scripts = &self.meta.scripts;
        assert!(scripts.len() <= 1);

        for script in scripts {
            self.lua.init(&script.code);
            self.lua.start();
        }
    }
}

/// Recursively iterates through all objects in the capsule view
pub fn iter_all_objects<F>(capsule: &Capsule, mut cb: F)
where
    F: FnMut(&ConcurrentElement<BoxedCapsuleObject>),
{
    fn recurse<F>(object: &ConcurrentElement<BoxedCapsuleObject>, cb: &mut F)
    where
        F: FnMut(&ConcurrentElement<BoxedCapsuleObject>),
    {
        cb(object);

        let base = object.map(|o| o.base());

        for obj in base.children.iter() {
            recurse(obj, cb);
        }
    }

    let base = capsule.view.base();

    for obj in base.children.iter() {
        recurse(obj, &mut cb);
    }
}
