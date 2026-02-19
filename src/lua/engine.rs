use std::sync::Arc;

use mlua::{FromLuaMulti, IntoLuaMulti, Lua, StdLib};

use crate::lua::modules::get_capsule_module;

#[derive(Debug, Default)]
pub struct LuaEngine {
    lua: Arc<Lua>,
    code: String,
}

impl LuaEngine {
    pub fn init(&mut self, code: &str) {
        let globals = self.lua.globals();
        self.lua.load_std_libs(StdLib::ALL_SAFE).unwrap();
        globals
            .set("capsule", get_capsule_module(&self.lua).unwrap())
            .unwrap();
        code.clone_into(&mut self.code);
    }

    pub fn start(&mut self) {
        self.lua.load(self.code.clone()).exec().unwrap();
    }

    pub fn call<R: FromLuaMulti>(&mut self, fun: impl IntoLuaMulti) -> mlua::Result<R> {
        self.lua.load(self.code.clone()).call::<R>(fun)
    }
}
