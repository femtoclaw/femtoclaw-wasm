# FemtoClaw WASM

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Experimental-orange.svg)]()

FemtoClaw WASM Runtime — WebAssembly sandboxing for isolated capability execution.

## Overview
`femtoclaw-wasm` provides WebAssembly-based sandbox execution for FemtoClaw capabilities. It enables secure, isolated execution of third-party capabilities without compromising host system integrity.

## Features
- **Sandboxed Execution**: Run capabilities in isolated WASM environments.
- **Resource Constraints**: Strictly enforce memory and CPU limits per capability.
- **Host Function Gateway**: Controlled access to host resources (FS, Net) via explicit imports.
- **Standardized Interface**: Common ABI for WASM-based Talons.

## Installation
```toml
[dependencies]
femtoclaw-wasm = "1.0.3"
```

## Related Specifications
- [FC-EXT-0001: Extension and Plugin Specification](../femtoclaw-spec/FC-EXT-0001-Extension_and_Plugin_Specification.md)
- [FC-07: Security Architecture](../femtoclaw-spec/07-FemtoClaw_Security_Architecture_Specification.md)

## License
Apache 2.0 — see [LICENSE](LICENSE).
