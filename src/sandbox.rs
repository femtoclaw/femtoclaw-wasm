//! WASM Sandbox.

use thiserror::Error;
use wasmtime::{Engine, Instance, Linker, Module as WasmModule, Store};

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("execution error: {0}")]
    Execution(String),
    #[error("linker error: {0}")]
    Linker(String),
}

pub struct Sandbox {
    engine: Engine,
    linker: Linker<()>,
}

impl Sandbox {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        Self { engine, linker }
    }

    pub fn add_host_function(
        &mut self,
        name: &str,
        func: impl Fn(&mut Store<()>) + Send + Sync + 'static,
    ) -> Result<(), SandboxError> {
        wasmtime::Func::wrap(&self.engine, func);
        Ok(())
    }

    pub fn execute(&self, module: &[u8], entry: &str) -> Result<String, SandboxError> {
        let module = WasmModule::new(&self.engine, module)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        Ok("executed".to_string())
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}
