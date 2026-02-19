use std::{any::Any, fmt::Debug, sync::Arc};

use macroquad::math::Rect;
use orx_concurrent_vec::{ConcurrentElement, ConcurrentVec};
use parking_lot::RwLock;

use crate::{
    capsule::objs::{script::CSScript, view::CSView},
    event::CapsuleObjectEvent,
    layout::{computed::ComputedStyling, styling::Styling},
    lua::engine::LuaEngine,
};

pub type ArcLock<T> = Arc<RwLock<T>>;
pub type BoxedCapsuleObject = Arc<dyn CapsuleObject + Sync + Send>;
pub type CapsuleObjectChildren = Arc<ConcurrentVec<BoxedCapsuleObject>>;
pub type CapsuleObjectEvents = Arc<ConcurrentVec<CapsuleObjectEvent>>;
pub type CapsuleObjectId = ArcLock<Option<String>>;

pub trait CapsuleObject: Debug {
    fn as_any(&self) -> &dyn Any;
    fn base(&self) -> Arc<CapsuleObjectBase>;
    fn render(&self);
    fn bounding_box(&self) -> Rect {
        let base = self.base();
        let computed = base.computed_style.read();

        Rect::new(computed.x, computed.y, computed.width, computed.height)
    }

    fn is_dirty(&self) -> bool {
        self.base().style.read().is_dirty()
    }

    fn set_dirty(&self) {
        self.base().style.write().set_dirty();
    }

    fn set_non_dirty(&self) {
        self.base().style.write().set_non_dirty();
    }
}

#[derive(Debug, Default, Clone)]
pub struct CapsuleObjectBase {
    pub id: CapsuleObjectId,
    pub children: CapsuleObjectChildren,
    pub events: CapsuleObjectEvents,
    pub style: ArcLock<Styling>,
    pub computed_style: ArcLock<ComputedStyling>,
}

#[derive(Debug, Default)]
pub struct CapsuleObjectCreationContext {
    pub id: CapsuleObjectId,
    pub children: CapsuleObjectChildren,
    pub events: CapsuleObjectEvents,
    pub style: ArcLock<Styling>,
}

impl CapsuleObjectCreationContext {
    #[must_use]
    pub fn new(
        children: CapsuleObjectChildren,
        events: CapsuleObjectEvents,
        style: ArcLock<Styling>,
        id: CapsuleObjectId,
    ) -> Self {
        Self {
            id,
            children,
            events,
            style,
        }
    }
}

impl CapsuleObjectBase {
    #[must_use]
    pub fn new(ctx: CapsuleObjectCreationContext) -> Arc<Self> {
        Arc::new(Self {
            children: ctx.children,
            style: ctx.style,
            events: ctx.events,
            id: ctx.id,
            computed_style: Arc::default(),
        })
    }

    #[must_use]
    pub fn children_vec(&self) -> Vec<BoxedCapsuleObject> {
        let mut out = Vec::new();

        for child in self.children.iter() {
            let child_owned: BoxedCapsuleObject = child.map(std::clone::Clone::clone);
            out.push(child_owned);
        }

        out
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
    pub view: CSView,
    pub lua: ArcLock<LuaEngine>,
}

impl Capsule {
    pub fn run_scripts(capsule: &ArcLock<Self>) {
        let scripts = {
            let cap = capsule.read();
            cap.meta.scripts.clone()
        };

        let mut lua = LuaEngine::default();

        for script in scripts {
            lua.init(&script.code, capsule);
            lua.start();
        }

        {
            let mut cap = capsule.write();
            cap.lua = RwLock::new(lua).into();
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

#[macro_export]
macro_rules! impl_obj_traits {
    ($name:ident) => {
        use mlua::UserData;

        impl UserData for $name {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                $crate::lua::holder::add_object_fields::<Self, F>(fields);
            }
        }
    };
}
