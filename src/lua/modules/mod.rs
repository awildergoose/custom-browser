#![allow(clippy::unnecessary_wraps)]
use mlua::prelude::*;

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

pub fn get_capsule_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("used_memory", lua.create_function(used_memory)?)?;
    Ok(exports)
}
