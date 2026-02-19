use std::sync::Arc;

use crate::capsule::obj::CapsuleObject;
use mlua::{UserData, Value};

#[derive(Debug, Clone)]
pub struct CapsuleObjectHandle(pub Arc<dyn CapsuleObject + Send + Sync>);

impl UserData for CapsuleObjectHandle {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        crate::lua::holder::add_object_fields::<Self, F>(fields);
    }
}

pub fn add_object_fields<T, F>(fields: &mut F)
where
    T: CapsuleObject + 'static,
    F: mlua::UserDataFields<T>,
{
    fields.add_field_method_get("children", |lua, this: &T| {
        let children = this.base().children_vec();
        let table = lua.create_table()?;

        for (i, child) in children.iter().enumerate() {
            let ud = lua.create_userdata(CapsuleObjectHandle(child.clone()))?;
            #[allow(clippy::cast_possible_wrap)]
            table.set((i + 1) as i64, ud)?;
        }

        Ok(Value::Table(table))
    });

    fields.add_field_method_get("width", |lua, this: &T| {
        Ok(Value::Number(
            this.base().computed_style.read().width.into(),
        ))
    });
}

impl CapsuleObject for CapsuleObjectHandle {
    fn base(&self) -> Arc<crate::capsule::obj::CapsuleObjectBase> {
        self.0.base()
    }

    fn render(&self) {
        self.0.render();
    }
}
