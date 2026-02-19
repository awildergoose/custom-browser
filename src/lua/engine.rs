use std::sync::Arc;

use log::Log;
use mlua::{Function, Lua, StdLib, Value};

use crate::{
    capsule::{Capsule, obj::ArcLock},
    lua::modules::get_capsule_module,
};

#[derive(Debug, Default, Clone)]
pub struct LuaLogger {}

impl Log for LuaLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("[LUA] {}", record.args());
        }
    }

    fn flush(&self) {}
}

#[derive(Debug, Default)]
pub struct LuaEngine {
    lua: Arc<Lua>,
    code: String,
    lua_logger: LuaLogger,
}

impl LuaEngine {
    pub fn init(&mut self, code: &str, capsule: &ArcLock<Capsule>) {
        let globals = self.lua.globals();
        self.lua.load_std_libs(StdLib::ALL_SAFE).unwrap();
        globals
            .set("capsule", get_capsule_module(&self.lua, capsule).unwrap())
            .unwrap();
        let lua_logger = self.lua_logger.clone();
        globals
            .set(
                "print",
                self.lua
                    .create_function(move |_lua, value: Value| {
                        let text = match value {
                            Value::Nil => "nil".to_string(),
                            Value::Boolean(b) => b.to_string(),
                            Value::Integer(i) => i.to_string(),
                            Value::Number(n) => n.to_string(),
                            Value::String(s) => s
                                .to_str()
                                .map_or_else(|_| "<invalid utf8>".to_owned(), |s| s.to_string()),
                            Value::UserData(_) => "[userdata]".to_string(),
                            other => format!("{other:?}"),
                        };

                        log::debug!(logger: lua_logger, "{text}");
                        Ok(())
                    })
                    .unwrap(),
            )
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
