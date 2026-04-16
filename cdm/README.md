# `cdm`

Low level access to the CDM-16 processor.

## Features
This crate provides:
- Access to special registers: `PSR`, `SP`, `FP`, `PC`.
- Interrupt control functions.
- Wrappers around assembly instructions like `wait` and `halt`.

## Optional features
#### `critical-section`
This feature enables a [`critical-section`](https://github.com/rust-embedded/critical-section)
implementation based on disabling interrupts globally.

#### `panic-halt`
This feature enables a panic handler that halts the processor with the `halt` instruction.
