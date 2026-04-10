//! WASM Sandbox.

use thiserror::Error;
use wasmtime::{Engine, Instance, Module as WasmModule, Store};

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("execution error: {0}")]
    Execution(String),
    #[error("linker error: {0}")]
    Linker(String),
    #[error("function not found: {0}")]
    FunctionNotFound(String),
}

pub struct Sandbox {
    engine: Engine,
}

impl Sandbox {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self { engine }
    }

    pub fn execute(&self, module_bytes: &[u8], entry: &str) -> Result<String, SandboxError> {
        let module = WasmModule::new(&self.engine, module_bytes)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let func_name = if entry.is_empty() { "_start" } else { entry };

        match instance.get_func(&mut store, func_name) {
            Some(func) => {
                func.call(&mut store, &[], &mut [])
                    .map_err(|e| SandboxError::Execution(e.to_string()))?;
                Ok("WASM executed successfully".to_string())
            }
            None => Err(SandboxError::FunctionNotFound(func_name.to_string())),
        }
    }

    pub fn execute_with_memory(
        &self,
        module_bytes: &[u8],
        func_name: &str,
        input: &str,
    ) -> Result<String, SandboxError> {
        let module = WasmModule::new(&self.engine, module_bytes)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let mut store = Store::new(&self.engine, ());

        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| SandboxError::Execution("memory not found".to_string()))?;

        let input_bytes = input.as_bytes();
        let ptr = memory.data_size(&store);
        memory
            .write(&mut store, ptr, input_bytes)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        if let Some(func) = instance.get_func(&mut store, func_name) {
            let mut result = [wasmtime::Val::I32(0)];
            func.call(&mut store, &[wasmtime::Val::I32(ptr as i32)], &mut result)
                .map_err(|e| SandboxError::Execution(e.to_string()))?;
            Ok(format!("Executed with memory at offset {}", ptr))
        } else {
            Err(SandboxError::FunctionNotFound(func_name.to_string()))
        }
    }

    pub fn list_functions(&self, module_bytes: &[u8]) -> Result<Vec<String>, SandboxError> {
        let module = WasmModule::new(&self.engine, module_bytes)
            .map_err(|e| SandboxError::Execution(e.to_string()))?;

        let mut exports = Vec::new();
        for export in module.exports() {
            if let Some(name) = export.name().strip_prefix("func.") {
                exports.push(name.to_string());
            } else if export.name() == "_start" {
                exports.push("_start".to_string());
            }
        }

        Ok(exports)
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}
