//! WASM Runtime.

use crate::module::Module;
use wasmtime::{Engine, Module as WasmModule, Store};

pub struct WasmRuntime {
    engine: Engine,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self { engine }
    }

    pub fn compile(&self, module: &Module) -> Result<wasmtime::Module, wasmtime::Error> {
        WasmModule::new(&self.engine, module.bytes())
    }

    pub fn instantiate(
        &self,
        module: &wasmtime::Module,
    ) -> Result<wasmtime::Instance, wasmtime::Error> {
        let mut store = Store::new(&self.engine, ());
        wasmtime::Instance::new(&mut store, module, &[])
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new()
    }
}
