#![allow(clippy::unnecessary_wraps)]
use std::sync::Arc;

use mlua::prelude::*;

use crate::{
    capsule::{
        Capsule,
        obj::{ArcLock, iter_all_objects},
        objs::view::CSView,
    },
    lua::holder::CapsuleObjectHandle,
};

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn get_root(_lua: &Lua, capsule: &ArcLock<Capsule>) -> LuaResult<CSView> {
    Ok(capsule.read().view.clone())
}

pub fn get_capsule_module(lua: &Lua, capsule: &ArcLock<Capsule>) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("used_memory", lua.create_function(used_memory)?)?;
    let capsule_c = Arc::clone(capsule);
    exports.set(
        "root",
        lua.create_function(move |lua: &Lua, (): ()| get_root(lua, &capsule_c))?,
    )?;
    let capsule_c = Arc::clone(capsule);
    exports.set(
        "find_element",
        lua.create_function(move |_lua: &Lua, id: String| {
            let mut found = None;

            iter_all_objects(&capsule_c.read(), |o| {
                if o.map(|o| o.base().id.read().as_deref() == Some(id.as_str())) {
                    found = Some(CapsuleObjectHandle(o.map(std::clone::Clone::clone)));
                }
            });

            Ok(found)
        })?,
    )?;

    Ok(exports)
}
