# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.2] - 2026-02-26

### Added
- Proper WASM function execution with memory support
- execute_with_memory for passing input to WASM modules
- list_functions to enumerate WASM exports

### Changed
- Version bump from 1.0.1 to 1.0.2

## [1.0.1] - 2026-02-25

### Added
- Sandbox for isolated execution
- Module loader for WASM modules
- WasmRuntime for running WASM-based capabilities
- Simplified API for ease of use

### Changed
- Version bump from 1.0.0 to 1.0.1
- Simplified sandbox API due to wasmtime compatibility

### Fixed
- Build errors resolved (wasmtime API issues)

## [1.0.0] - 2026-02-25

### Added
- Initial release of femtoclaw-wasm
- WASM sandbox functionality
