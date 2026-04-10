# FemtoClaw WASM

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Experimental-orange.svg)]()

FemtoClaw WASM Runtime — WebAssembly sandbox execution for isolated capability execution.

## Overview

`femtoclaw-wasm` provides WebAssembly-based sandbox execution for FemtoClaw capabilities. It enables secure, isolated execution of user-defined or third-party capabilities without compromising the runtime.

This library is designed according to the [FemtoClaw Extension Specification (FC-EXT-0001)](../femtoclaw-spec/FC-EXT-0001-Extension_and_Plugin_Specification.md).

## Features

- **Sandboxed Execution**: Run capabilities in isolated WASM environments
- **WASMtime Backend**: High-performance WebAssembly runtime
- **Resource Limits**: Configurable memory and CPU limits
- **Host Functions**: Controlled access to host resources
- **Module Management**: Load and instantiate WASM modules

## Security Model

```
┌─────────────────────────────────────────────────────────────┐
│                    FemtoClaw Runtime                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐     │
│  │              WASM Sandboxes                         │     │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐        │     │
│  │  │ Capability│  │ Capability│  │ Capability│        │     │
│  │  │   WASM    │  │   WASM    │  │   WASM    │        │     │
│  │  │  Module   │  │  Module   │  │  Module   │        │     │
│  │  └───────────┘  └───────────┘  └───────────┘        │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐     │
│  │         Host Function Gateway                        │     │
│  │  - Filesystem: read, write (with path restrictions) │     │
│  │  - Network: HTTP requests (allowlist only)          │     │
│  │  - Logging: structured output                        │     │
│  └─────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## Installation

```toml
[dependencies]
femtoclaw-wasm = "1.0"
```

## Usage

```rust
use femtoclaw_wasm::{Sandbox, Module};

// Load WASM module
let module_bytes = std::fs::read("capability.wasm")?;
let module = Module::from_bytes(module_bytes);

// Create sandbox with resource limits
let mut sandbox = Sandbox::new();
sandbox.set_memory_limit(64 * 1024 * 1024); // 64MB
sandbox.set_cpu_limit(std::time::Duration::from_secs(5));

// Execute
let result = sandbox.execute(&module_bytes, "run")?;
println!("Result: {}", result);
```

## WASM Module Requirements

WASM capabilities must expose a standard interface:

```rust
// Expected exports
#[no_mangle]
pub extern "C" fn run(input: *const u8, input_len: usize) -> *const u8;
```

## Sandbox Capabilities

When running in aFemtoClaw WASM sandbox, modules have access to:

| Capability | Description |
|------------|-------------|
| `femto.log(message)` | Write to structured log |
| `femto.http.request()` | Make HTTP requests (restricted) |
| `femto.fs.read()` | Read files (restricted path) |
| `femto.result(value)` | Return result to runtime |

## Limitations

- No direct filesystem access (must use host functions)
- No network by default (must be explicitly allowed)
- No process spawning
- Memory limits enforced
- Execution time limits enforced

## Dependencies

- wasmtime 32.x
- serde 1.x
- serde_json 1.x
- thiserror 1.x
- tokio 1.x (non-WASM targets)

## Requirements

- Rust 1.75 or later
- For native: standard Rust toolchain
- For WASM: wasm32-unknown-unknown target

## Related Specifications

- [FC-EXT-0001: Extension and Plugin Specification](../femtoclaw-spec/FC-EXT-0001-Extension_and_Plugin_Specification.md)
- [FC-07: Security Architecture](../femtoclaw-spec/07-FemtoClaw_Security_Architecture_Specification.md)

## Related Crates

| Crate | Purpose |
|-------|---------|
| `femtoclaw` | Core runtime |
| `femtoclaw-policy` | Authorization |
| `femtoclaw-claws` | Built-in capabilities |

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
