#![allow(clippy::unnecessary_wraps)]
use std::sync::Arc;

use mlua::prelude::*;

use crate::capsule::{Capsule, obj::ArcLock, objs::view::CapsuleView};

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn get_root(_lua: &Lua, capsule: &ArcLock<Capsule>) -> LuaResult<CapsuleView> {
    Ok(capsule.read().view.clone())
}

pub fn get_capsule_module(lua: &Lua, capsule: &ArcLock<Capsule>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("used_memory", lua.create_function(used_memory)?)?;
    let capsule = Arc::clone(capsule);
    exports.set(
        "root",
        lua.create_function(move |lua: &Lua, (): ()| get_root(lua, &capsule))?,
    )?;
    Ok(exports)
}
