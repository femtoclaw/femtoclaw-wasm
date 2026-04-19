# 🕸️ FemtoClaw WASM: Sandboxed Capability Execution

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Experimental-orange.svg)]()

The **FemtoClaw WASM** crate provides a high-performance WebAssembly-based sandbox for isolated capability execution. By utilizing WebAssembly, FemtoClaw can execute third-party or user-defined "Skills" without compromising the integrity of the host runtime or the operating system.

This library implements the **Extension and Plugin Specification (FC-EXT-0001)**, providing a secure bridge between the deterministic Rust runtime and external logic.

---

## 🛡️ Security & Isolation Model

The primary mission of `femtoclaw-wasm` is to provide **Zero-Trust Execution**. Unlike native capabilities that run with the same privileges as the runtime process, WASM capabilities are confined to a strictly bounded environment.

### 1. Fault Isolation
If a WASM capability crashes (e.g., out-of-memory, panic, or infinite loop), the main FemtoClaw runtime remains unaffected. The sandbox is terminated, and a deterministic error is returned to the execution loop.

### 2. Resource Constraints
Every WASM sandbox is initialized with explicit resource limits:
- **Memory Limits**: Prevents a capability from exhausting host RAM.
- **CPU Off-ramps**: Prevents infinite loops via deterministic "gas" or instruction counting.
- **Stack Protection**: Guards against deep recursion attacks.

### 3. Deny-by-Default Host Functions
WASM modules have NO access to the outside world by default. Access to the filesystem, network, or environment variables is provided ONLY through an explicit **Host Function Gateway**.

---

## 🏗️ Technical Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                    FemtoClaw Core Runtime                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              WASM Sandbox (Wasmtime)                │    │
│  │  ┌───────────────┐  ┌───────────────┐               │    │
│  │  │  Third-Party  │  │  User-Defined │               │    │
│  │  │    Skill      │  │    Action     │               │    │
│  │  └───────────────┘  └───────────────┘               │    │
│  └──────────┬───────────────────┬──────────────────────┘    │
│             │                   │                           │
│  ┌──────────▼───────────────────▼──────────────────────┐    │
│  │            Host Function Gateway                    │    │
│  │  - Controlled I/O (Spec 04)                         │    │
│  │  - Authorization Check (Spec 05)                    │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 Usage Guide

### 1. Embedding the WASM Sandbox
```rust
use femtoclaw_wasm::{Sandbox, Module};

// Load a compiled .wasm skill
let wasm_bytes = std::fs::read("my_capability.wasm")?;
let module = Module::from_bytes(wasm_bytes);

// Create a sandbox with strict 64MB memory limit
let mut sandbox = Sandbox::new();
sandbox.set_memory_limit(64 * 1024 * 1024); 

// Execute the 'run' export
let result = sandbox.execute(&module, "run", json!({"input": "test"}))?;
println!("WASM Output: {}", result);
```

### 2. Developing a WASM Skill
Skill authors utilize the `femtoclaw-sdk` to export the required interface.
```rust
// In your WASM Skill crate
#[no_mangle]
pub extern "C" fn run(input_ptr: *const u8, input_len: usize) -> *const u8 {
    // Implementation logic here
}
```

---

## 🧱 Standard ABI (Application Binary Interface)

To ensure compatibility across different runtime versions, all WASM-based FemtoClaw extensions MUST conform to the **FC-ABI-0001** specification:
- **Encoding**: All inputs and outputs must be JSON-serialized.
- **Imports**: Modules may only import host functions prefixed with `femto_`.
- **Memory**: Linear memory is managed by the host to ensure safe data transfers.

---

## 📄 Related Specifications
- **[FC-EXT-0001: Extension and Plugin Specification](../femtoclaw-spec/FC-EXT-0001-Extension_and_Plugin_Specification.md)**
- **[FC-07: Security Architecture](../femtoclaw-spec/07-FemtoClaw_Security_Architecture_Specification.md)**

Copyright © 2026 FemtoClaw Project.
