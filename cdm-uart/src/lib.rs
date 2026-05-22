//! Hardware abstraction for Logisim UART
//!
#![cfg_attr(all(), doc = embed_doc_image::embed_image!("logisim", "images/logisim.png"))]
//!
//! # Features
//! This crate provides:
//! - basic hardware abstraction for the [Logisim UART component](https://github.com/cdm-processors/logisim-uart)
//! - implementation of [`embedded-io`](https://github.com/rust-embedded/embedded-hal/tree/master/embedded-io) traits
//! - implementation of `core::fmt::Write`
//!
//! # Requirements
//! The crate uses two 8-bit memory-mapped I/O registers to access the UART component:
//! - flags (read only):
//!   - bit 0 (LSB) - connection active
//!   - bit 1 - read buffer not empty
//!   - bits 2-7 - reserved
//! - data (read and write):
//!   - read - get and remove byte from the read queue
//!   - write - put byte into the write queue
//!
//! The component needs to wired up correctly. Use this image as a reference:
//!
//! ![Logisim UART connection schematic][logisim]
//!
//! - `flags enable` and `data enable` are high when `memory` on the processor is high and the value on the address
//! bus matches the address of the respective register
//! - `read/write` is connected directly to `read/write` on the processor.
//!
//! # Example
//! ```rust
//! use cdm_uart::Uart;
//! use core::fmt::Write;
//!
//! unsafe extern "C" {
//!   static UART_FLAGS: u8;
//!   static mut UART_RX_TX: u8;
//! }
//!
//! fn uart() -> Uart {
//!   unsafe {
//!     Uart::new(&raw const UART_FLAGS, &raw mut UART_RX_TX)
//!   }
//! }
//!
//! fn example() {
//!   let _ = writeln!(uart(), "Hello, world!");
//! }
//! ```
//! Linker script:
//! ```ld
//! UART_FLAGS = /* flags register address */
//! UART_RX_TX = /* rx/tx register address */
//! ```

#![no_std]
#![feature(asm_experimental_arch)]
#![doc(html_logo_url = "https://aelsi2.github.io/cdm-rs/logo.png")]

mod corefmt;
mod embedded_io;
mod uart;

pub use uart::Uart;
pub use uart::UartError;
