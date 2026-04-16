//! Low level access to the CDM-16 processor.
//!
//! # Features
//!
//! This crate provides:
//! - Access to special registers: `PSR`, `SP`, `FP`, `PC`
//! - Interrupt control functions
//! - Wrappers around assembly instructions like `wait` and `halt`
//! - An optional `critical-section` implementation
//! - An optional panic handler
//!
//! # Cargo features
//! #### `critical-section`
//! This feature enables a [`critical-section`](https://github.com/rust-embedded/critical-section)
//! implementation based on disabling interrupts globally.
//! ***
//!
//! #### `panic-halt`
//! This feature enables a panic handler that halts the processor with the `halt` instruction.

#![no_std]
#![feature(asm_experimental_arch)]

pub mod interrupt;
pub mod register;
pub mod execution;

#[cfg(feature = "critical-section")]
mod critical_section;
#[cfg(feature = "panic-halt")]
mod panic_halt;
