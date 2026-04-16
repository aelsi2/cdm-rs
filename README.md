# CDM-16 hardware support crates

This repository contains a set of crates for accessing the hardware features of the CDM-16 processor:
- `cdm` - register access, optional implementation of [critical-section](https://crates.io/crates/critical-section), optional panic handler
- `cdm-rt` - startup code, default exception handlers, interrupt handler definition utilities
- `cdm-macros` - procedural macros for defining the entry point of the program, interrupt and exception handlers, re-exported in `cdm-rt`

These crates are made to be used with the experimental [Rust compiler for CDM-16](https://github.com/ylab-nsu/cdm16-rust) based on the [CDM-16 LLVM fork](https://github.com/ylab-nsu/cdm16-llvm-neo/).

- [**Documentation**](https://aelsi2.github.io/cdm-rs/index.html)
- [**Example project**](https://github.com/aelsi2/cdm-paint-rs)

## How to use
Add this to your project's Cargo.toml:
```toml
[dependencies.cdm]
git = "https://github.com/aelsi2/cdm-rs.git"
features = [
    # Optional: adds a panic handler that uses CDM-16's halt instruction.
    # "panic-halt",
    # Optional: adds an implementation of critical-section that disables interrupts.
    # "critical-section",
] 

[dependencies.cdm-rt]
git = "https://github.com/aelsi2/cdm-rs.git"
features = [
    # Optional: disables default interrupt handlers and allows definition 
    # of custom interrupt handlers with interrupt_vectors![...]
    # "interrupts",
]
```

For a minimal working program, mark your `main` function with the `cdm_rt::entry` attribute:
```rust
// main.rs
use cdm_rt::entry;

#[entry]
fn main() -> ! {
    loop { /* .. */ }
}
```
