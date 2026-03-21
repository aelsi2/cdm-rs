# `cdm-rt` macros

This crate contains procedural macros for functions re-exported in `cdm-rt`:
- `#[entry]` - defines the entry point of the application
- `#[exception(/* type */)]` - defines an exception handler for the specified exception type; exception types:
    - `#[exception(Default)]` - default exception handler used as a fallback
    - `#[exception(UnalignedSP)]` - unaligned stack pointer
    - `#[exception(UnalignedPC)]` - unaligned program counter
    - `#[exception(InvalidInst)]` - invalid instruction
    - `#[exception(DoubleFault)]` - double fault
- `#[interrupt]` - declares the function as an interrupt handler (but does not add it to the IVT, use `cdm_rt::interrupt_vectors![]` for this)

Entry point and exception handler functions must not return (e.g. must have `!` return type).

### Examples
Entry point:
```rust
use cdm_rt::entry;

#[entry]
fn main() -> ! {
    loop { /* .. */ }
}
```

Exception handlers:
```rust
use cdm_rt::exception;

#[exception(Default)]
fn on_exception() -> ! {
    loop { /* .. */ }
}

#[exception(InvalidInst)]
fn on_invalid_inst() -> ! {
    loop { /* .. */ }
}
```

Interrupt handlers:
```rust
use cdm_rt::{InterruptVector, Psr, interrupt_vectors};
use cdm_rt::interrupt;

interrupt_vectors![
    InterruptVector(on_interrupt, Psr::None),
];

#[interrupt]
fn on_interrupt() {
    /* .. */
}
```
