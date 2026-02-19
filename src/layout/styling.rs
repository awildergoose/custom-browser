use anyhow::Context;
use mlua::{UserData, Value};
use parking_lot::{RawRwLock, lock_api::RwLockWriteGuard};
use serde::{Deserialize, Serialize};

use crate::{
    capsule::{
        obj::ArcLock,
        parser::{try_parse_color, try_parse_dimension},
    },
    layout::capsule::{
        align::COAlignItems, color::COColor, dimension::CODimension, flexdir::COFlexDirection,
        justify::COJustifyContent,
    },
    renderer::constants::DEFAULT_TEXT_SIZE,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Styling {
    pub align: COAlignItems,
    pub justify: COJustifyContent,
    pub flexdir: COFlexDirection,

    pub width: Option<CODimension>,
    pub height: Option<CODimension>,
    pub color: Option<COColor>,
    pub background_color: Option<COColor>,
    pub font_size: u16,

    dirty: bool,
}

impl Styling {
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub const fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub const fn set_non_dirty(&mut self) {
        self.dirty = false;
    }
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            align: COAlignItems::default(),
            justify: COJustifyContent::default(),
            flexdir: COFlexDirection::default(),
            width: None,
            height: None,
            font_size: DEFAULT_TEXT_SIZE,
            color: None,
            background_color: None,
            dirty: false,
        }
    }
}

#[derive(Clone)]
pub struct StylingHandle(pub ArcLock<Styling>);

impl StylingHandle {
    pub fn write(&self) -> RwLockWriteGuard<'_, RawRwLock, Styling> {
        let mut writer = self.0.write();
        writer.set_dirty();
        writer
    }
}

macro_rules! impl_setget_dimension {
    ($fields: ident, $name: ident) => {
        $fields.add_field_method_get(stringify!($name), |lua, this| {
            let value = this.0.read().$name;
            if let Some(value) = value {
                return Ok(Value::String(lua.create_string(value.as_text())?));
            }

            Ok(Value::Nil)
        });

        $fields.add_field_method_set(stringify!($name), |_lua, this, v: String| {
            this.0.write().$name = try_parse_dimension(&v);
            this.0.write().set_dirty();
            Ok(())
        });
    };
}

macro_rules! impl_setget_enum {
    ($fields: ident, $name: ident, $type: tt) => {
        $fields.add_field_method_get(stringify!($name), |lua, this| {
            Ok(Value::String(
                lua.create_string(this.0.read().$name.as_ref())?,
            ))
        });

        $fields.add_field_method_set(stringify!($name), |_lua, this, v: String| {
            let value = v
                .parse::<$type>()
                .context(format!("failed to parse {}", stringify!($name)))?;
            this.write().$name = value;
            Ok(())
        });
    };
}

macro_rules! impl_setget_primitive {
    ($fields: ident, $name: ident, $type: tt, $lua_type: tt) => {
        $fields.add_field_method_get(stringify!($name), |_lua, this| {
            Ok(Value::$lua_type(this.0.read().$name.into()))
        });

        $fields.add_field_method_set(stringify!($name), |_lua, this, v: $type| {
            this.write().$name = v;
            Ok(())
        });
    };
}

macro_rules! impl_setget_color {
    ($fields: ident, $name: ident) => {
        $fields.add_field_method_get(stringify!($name), |lua, this| {
            let value = this.0.read().$name;
            if let Some(value) = value {
                return Ok(Value::String(lua.create_string(value.as_str())?));
            }

            Ok(Value::Nil)
        });

        $fields.add_field_method_set(stringify!($name), |_lua, this, color: String| {
            this.write().$name = try_parse_color(&color);
            Ok(())
        });
    };
}

impl UserData for StylingHandle {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        impl_setget_dimension!(fields, width);
        impl_setget_dimension!(fields, height);
        impl_setget_enum!(fields, align, COAlignItems);
        impl_setget_enum!(fields, justify, COJustifyContent);
        impl_setget_enum!(fields, flexdir, COFlexDirection);
        impl_setget_primitive!(fields, font_size, u16, Number);
        impl_setget_color!(fields, color);
        impl_setget_color!(fields, background_color);
    }
}
