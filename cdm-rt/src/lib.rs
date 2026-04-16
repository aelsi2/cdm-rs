//! Startup code and minimal runtime for the CDM-16 processor.
//!
//! # Features
//!
//! This crate provides:
//! - Default interrupt and exception handlers using the `halt` instruction
//! - A macro for defining the entry point of the program: [`entry`](macro@entry)
//! - A macro for redefining exception handlers: [`exception`](macro@exception)
//! - A macro for defining interrupt handlers: [`interrupt`](macro@interrupt)
//! - A macro for registering interrupt handlers in the IVT:
//! [`interrupt_vectors`](macro@interrupt_vectors)
//!
//! # Cargo features
//! #### `interrupts`
//! This feature disables the default interrupt handlers and allows the use of [`interrupt_vectors`](macro@interrupt_vectors)
//!
//! # Requirements
//! #### `memory.x`
//! The crate expects a file named `memory.x` to be present in the project root directory.
//! This file is a linker script that needs to specify the address range that the linker can use.
//! This is done using the `MEMORY` command defining a region named `RAM`. The origin
//! address must be greater than or equal to `0x100` so that it doesn't intersect with the IVT.
//!
//! You can reserve a range of addresses between the IVT and the main memory for MMIO by specifying a
//! larger origin address for `RAM` (e.g `0x120`).
//! ***
//!
//! #### Rust flags
//! The crate generates a linker script named `link.x` in the output directory. It needs to be passed to the
//! linker. This can be done by passing `-Clink-arg=-Tlink.x` to rustc.
//! ***
//!
//! #### Entry point
//! Exactly one function needs to be marked as the application entry point by applying the [`entry`](macro@entry) attribute.
//!
//! # Example
//! `.cargo/config.toml`
//! ```toml
//! [build]
//! target = "cdm-none"
//! rustflags = [ "-Clink-arg=-Tlink.x" ]
//! [unstable]
//! build-std = [ "core" ]
//! ```
//!
//! `memory.x`
//! ```ld
//! MEMORY {
//!     RAM : ORIGIN = 0x100, LENGTH = 64K-0x100
//! }
//! ```
//!
//! `src/main.rs`
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! use cdm_rt::entry;
//!
//! #[entry]
//! fn main() -> ! {
//!     loop { /* .. */ }
//! }
//! ```

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(doc_auto_cfg)]

pub use cdm_macros::entry;

pub use cdm_macros::exception;

pub use cdm_macros::interrupt;

pub use cdm::register::psr::Psr;

/// The number of exception vectors in the interrupt vector table.
pub const EXCEPTION_COUNT: usize = 4;

/// The index of the first application-specific interrupt vector.
pub const INTERRUPT_START: usize = EXCEPTION_COUNT + 1;

/// The number of application-specific interrupt vectors in the interrupt vector table.
pub const INTERRUPT_COUNT: usize = 59;

/// Represents a vector in the interrupt vector table.
///
/// The first field is the pointer to the handler function.
/// The second field is the initial value of the processor status register.
///
/// Use `#[interrupt]` to define interrupt handler functions.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptVector(pub unsafe extern "cdm-isr" fn(), pub Psr);

impl InterruptVector {
    /// The default vector used in absence of an explicit definition.
    ///
    /// Calls `InterruptHandler`, which triggers a hardware halt by default.
    pub const DEFAULT: Self = {
        unsafe extern "cdm-isr" {
            fn InterruptHandler();
        }
        InterruptVector(InterruptHandler, Psr::None)
    };
}

/// Defines the application-specific interrupt handler section of the interrupt vector table.
///
/// The interrupt vectors specified in the arguments are placed sequentially after the reset and
/// exception vectors, starting from index `INTERRUPT_START`. The rest of the table is filled
/// with `InterruptVector::DEFAULT`.
///
/// Must be used **once** in the dependency graph.
///
/// Use the [`interrupt`](macro@interrupt) attribute to define interrupt handler functions.
///
/// ``` no_run
/// interrupt_vectors![
///     InterruptVector(MyHandler1, Psr::None), // int INTERRUPT_START+0
///     InterruptVector(MyHandler2, Psr::None), // int INTERRUPT_START+1
///     InterruptVector(MyHandler3, Psr::None), // int INTERRUPT_START+2
/// ];
///
/// #[cdm_rt::interrupt]
/// fn MyHandler1() { /* .. */ }
/// #[cdm_rt::interrupt]
/// fn MyHandler2() { /* .. */ }
/// #[cdm_rt::interrupt]
/// fn MyHandler3() { /* .. */ }
/// ```
#[cfg(feature = "interrupts")]
#[macro_export]
macro_rules! interrupt_vectors {
    ($($elems:expr),* $(,)?) => {
        #[used]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".ivt.interrupts")]
        static __INTERRUPTS: [$crate::InterruptVector; $crate::INTERRUPT_COUNT] = {
            const fn make_array<const N: usize, const M: usize>(
                prefix: [$crate::InterruptVector; M],
            ) -> [$crate::InterruptVector; N] {
                assert!(M <= N, "Prefix length cannot exceed array length");
                let mut arr = [$crate::InterruptVector::DEFAULT; N];
                let mut i: usize = 0;
                while i < M {
                    arr[i] = prefix[i];
                    i += 1;
                }
                arr
            }

            make_array([$($elems),*])
        };
    };
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ExceptionVector(pub unsafe extern "C" fn() -> !, pub Psr);

// The initialization code
core::arch::global_asm!(
    ".section .text._start",
    ".global _start",
    ".type _start,%function",
    "_start:",
    "ldi fp, 0",
    "stsp fp",
    "jsr main",
    "halt",
);

// The default interrupt and exception handler
core::arch::global_asm!(
    ".section .text._DefaultHandler",
    ".global _DefaultHandler",
    ".type _DefaultHandler,%function",
    "_DefaultHandler:",
    "halt",
);

unsafe extern "C" {
    #[link_name = "_start"]
    fn Reset() -> !;
    fn UnalignedSP() -> !;
    fn UnalignedPC() -> !;
    fn InvalidInst() -> !;
    fn DoubleFault() -> !;
}

// The reset vector
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.reset_vector")]
static __RESET_VECTOR: ExceptionVector = ExceptionVector(Reset, Psr::None);

// Harware-defined exception vectors
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.exceptions")]
static __EXCEPTIONS: [ExceptionVector; EXCEPTION_COUNT] = [
    ExceptionVector(UnalignedSP, Psr::None),
    ExceptionVector(UnalignedPC, Psr::None),
    ExceptionVector(InvalidInst, Psr::None),
    ExceptionVector(DoubleFault, Psr::None),
];

// Application-specific interrupt vectors
#[cfg(not(feature = "interrupts"))]
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.interrupts")]
static __INTERRUPTS: [InterruptVector; INTERRUPT_COUNT] =
    [InterruptVector::DEFAULT; INTERRUPT_COUNT];
