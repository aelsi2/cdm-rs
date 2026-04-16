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
#### `interrupts`
This feature disables the default interrupt handlers and allows the usage of `interrupt_vectors![]`.

## Minimal example
```rust
use cdm_rt::entry;
 
#[entry]
fn main() -> ! {
    loop { /* .. */ }
}
```
