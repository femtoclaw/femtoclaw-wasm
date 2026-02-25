//! WASM Sandbox.

use thiserror::Error;
use wasmtime::{Engine, Instance, Module as WasmModule, Store};

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("execution error: {0}")]
    Execution(String),
    #[error("linker error: {0}")]
    Linker(String),
}

pub struct Sandbox {
    engine: Engine,
}

impl Sandbox {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self { engine }
    }

    pub fn execute(&self, module: &[u8], _entry: &str) -> Result<String, SandboxError> {
        let module = WasmModule::new(&self.engine, module)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        let _instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        Ok("executed".to_string())
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}
