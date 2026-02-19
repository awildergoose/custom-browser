#![allow(clippy::unnecessary_wraps)]
use std::sync::Arc;

use mlua::prelude::*;
use parking_lot::RwLock;

use crate::capsule::{Capsule, objs::view::CapsuleView};

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn get_root(lua: &Lua, capsule: &Capsule) -> LuaResult<CapsuleView> {
    Ok(capsule.view.clone())
}

pub fn get_capsule_module(lua: &Lua, capsule: &Arc<RwLock<Capsule>>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("used_memory", lua.create_function(used_memory)?)?;
    let capsule = Arc::clone(capsule);
    exports.set(
        "get_root",
        lua.create_function(move |lua: &Lua, (): ()| {
            let capsule = capsule.read();
            get_root(lua, &capsule)
        })?,
    )?;
    Ok(exports)
}
