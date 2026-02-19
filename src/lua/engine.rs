use std::sync::Arc;

use mlua::{Function, Lua, StdLib};

use crate::{
    capsule::{Capsule, obj::ArcLock},
    lua::modules::get_capsule_module,
};

#[derive(Debug, Default)]
pub struct LuaEngine {
    lua: Arc<Lua>,
    code: String,
}

impl LuaEngine {
    pub fn init(&mut self, code: &str, capsule: &ArcLock<Capsule>) {
        let globals = self.lua.globals();
        self.lua.load_std_libs(StdLib::ALL_SAFE).unwrap();
        globals
            .set("capsule", get_capsule_module(&self.lua, capsule).unwrap())
            .unwrap();
        code.clone_into(&mut self.code);
    }

    pub fn start(&mut self) {
        if let Err(e) = self.lua.load(self.code.clone()).exec() {
            log::error!("Lua error: {e}");
        }
    }

    pub fn get_function(&mut self, name: &str) -> mlua::Result<Function> {
        self.lua.globals().get(name)
    }
}
