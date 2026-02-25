//! FemtoClaw WASM Runtime Library.
//!
//! Provides WebAssembly sandbox execution for isolated capability execution
//! according to FemtoClaw Extension Specification (FC-EXT-0001).

pub mod sandbox;
pub mod module;
pub mod runtime;

pub use sandbox::Sandbox;
pub use module::Module;
pub use runtime::WasmRuntime;
