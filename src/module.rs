//! WASM Module.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("compilation error: {0}")]
    Compile(String),
    #[error("instantiation error: {0}")]
    Instantiate(String),
}

pub struct Module {
    bytes: Vec<u8>,
}

impl Module {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
