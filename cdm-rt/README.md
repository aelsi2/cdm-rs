# `cdm-rt`

Startup code and minimal runtime for the CDM-16 processor.

## Features

This crate provides:
- Default interrupt and exception handlers using the `halt` instruction.
- A macro for defining the entry point of the program: `#[entry]`.
- A macro for redefining exception handlers: `#[exception]`.
- A macro for defining custom interrupt handlers: `#[interrupt]`.
- A macro for registering cuustom interrupt handlers: `interrupt_vectors![]`.

## Optional features
### `interrupts`
This feature disables the default interrupt handlers and allows the usage of `interrupt_vectors![]`.

## Requirements
### `memory.x`
The crate expects a file named `memory.x` to be present in the project root directory.
This file is a linker script that needs to specify the memory range that the linker can use.
This is done using the `MEMORY` command defining a region named `RAM`. The origin
address must be greater than or equal to `0x100` so that it doesn't intersect with the IVT.

You can reserve a range of addresses between the IVT and the main memory for MMIO by specifying a 
larger origin address for `RAM` (e.g `0x120`).

### Rust flags
The crate generates a linker script named `link.x` in the output directory. It needs to be passed to the
linker. This can be done by passing `-Clink-arg=-Tlink.x` to rustc.

### Entry point
Exactly one function needs to be marked as the application entry point by applying the
`#[cdm_rt::entry]` attribute.

## Example
`.cargo/config.toml`:
```toml
[build]
target = "cdm-none"
rustflags = [ "-Clink-arg=-Tlink.x" ]
[unstable]
build-std = [ "core" ]
```

`memory.x`:
```ld
MEMORY {
    RAM : ORIGIN = 0x100, LENGTH = 64K-0x100
}
```

`src/main.rs`:
```rust
use cdm_rt::entry;

#[entry]
fn main() -> ! {
    loop { /* .. */ }
}
```
