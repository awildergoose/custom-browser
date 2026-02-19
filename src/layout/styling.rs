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

impl UserData for StylingHandle {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        // TODO: these will all deadlock rn
        fields.add_field_method_get("width", |lua, this| {
            let width = this.0.read().width;
            if let Some(width) = width {
                return Ok(Value::String(lua.create_string(width.as_text())?));
            }

            Ok(Value::Nil)
        });

        fields.add_field_method_set("width", |_lua, this, v: String| {
            this.0.write().width = try_parse_dimension(&v);
            this.0.write().set_dirty();
            Ok(())
        });

        fields.add_field_method_get("height", |lua, this| {
            let height = this.0.read().height;
            if let Some(height) = height {
                return Ok(Value::String(lua.create_string(height.as_text())?));
            }

            Ok(Value::Nil)
        });

        fields.add_field_method_set("height", |_lua, this, v: String| {
            this.write().height = try_parse_dimension(&v);
            Ok(())
        });

        fields.add_field_method_get("align", |lua, this| {
            Ok(Value::String(
                lua.create_string(this.0.read().align.as_ref())?,
            ))
        });

        fields.add_field_method_set("align", |_lua, this, v: String| {
            let align = v.parse::<COAlignItems>().context("failed to parse align")?;
            this.write().align = align;
            Ok(())
        });

        fields.add_field_method_get("justify", |lua, this| {
            Ok(Value::String(
                lua.create_string(this.0.read().justify.as_ref())?,
            ))
        });

        fields.add_field_method_set("justify", |_lua, this, v: String| {
            let justify = v
                .parse::<COJustifyContent>()
                .context("failed to parse justify")?;
            this.write().justify = justify;
            Ok(())
        });

        fields.add_field_method_get("font_size", |_lua, this| {
            Ok(Value::Number(this.0.read().font_size.into()))
        });

        fields.add_field_method_set("font_size", |_lua, this, v: u16| {
            this.write().font_size = v;
            Ok(())
        });

        fields.add_field_method_get("color", |lua, this| {
            let color = this.0.read().color;
            if let Some(color) = color {
                return Ok(Value::String(lua.create_string(color.as_str())?));
            }

            Ok(Value::Nil)
        });

        fields.add_field_method_set("color", |_lua, this, color: String| {
            this.write().color = try_parse_color(&color);
            Ok(())
        });

        fields.add_field_method_get("flexdir", |lua, this| {
            Ok(Value::String(
                lua.create_string(this.0.read().flexdir.as_ref())?,
            ))
        });

        fields.add_field_method_set("flexdir", |_lua, this, v: String| {
            let flexdir = v
                .parse::<COFlexDirection>()
                .context("failed to parse flexdir")?;
            this.write().flexdir = flexdir;
            Ok(())
        });
    }
}
