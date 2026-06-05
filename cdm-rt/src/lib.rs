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
//! #### `harvard`
//! This feature enables generation of images for the [Harvard architecture](https://en.wikipedia.org/wiki/Harvard_architecture) with separate program and data address spaces.
//! By default, the [von Neumann architecture](https://en.wikipedia.org/wiki/Von_Neumann_architecture) with one address space is used.
//!
//! # Requirements
//! #### Rust flags
//! The crate generates a linker script named `link.x` in the output directory. It needs to be passed to the
//! linker. This can be done by passing `-Clink-arg=-Tlink.x` to rustc.
//!
//! #### Entry point
//! Exactly one function needs to be marked as the application entry point by applying the [`entry`](macro@entry) attribute.
//! ***
//!
//! #### Example
//! ##### `.cargo/config.toml`
//! ```toml
//! [build]
//! target = "cdm-none"
//! rustflags = [ "-Clink-arg=-Tlink.x" ]
//! [unstable]
//! build-std = [ "core" ]
//! ```
//!
//! ##### `src/main.rs`
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
//!
//! ## Memory mapped I/O
//! If you want to use memory mapped I/O, you will need to:
//! - create a linker script (e.g. `memory.x` in the project root)
//! - set the value of `RAM_ORIGIN` and/or `RAM_LENGTH` in the linker script to reserve a range of addresses for the MMIO registers
//! - create one or more symbols for the registers and set their values in the linker script
//! - pass the script to the linker (e.g. pass `-Clink-arg=-Tmemory.x` to rustc)
//! - declare the symbols in Rust inside an `extern "C"` block
//! - use [`core::ptr::read_volatile`](https://doc.rust-lang.org/core/ptr/fn.read_volatile.html) and/or 
//! [`core::ptr::write_volatile`](https://doc.rust-lang.org/core/ptr/fn.write_volatile.html) to read from and write
//! to the registers
//!
//! **Important note:** when using the von Neumann architecture, `RAM_ORIGIN` and MMIO register
//! addresses need to be greater or equal to `0x100` (256 in decimal) to avoid overlapping with the IVT.
//! ***
//!
//! #### Example
//! ##### `.cargo/config.toml`
//! ```toml
//! [build]
//! target = "cdm-none"
//! rustflags = [ "-Clink-arg=-Tlink.x", "-Clink-arg=-Tmemory.x" ]
//! [unstable]
//! build-std = [ "core" ]
//! ```
//!
//! ##### `memory.x`
//! ```ld
//! RAM_ORIGIN = 0x120;
//!
//! MMIO_IN = 0x100;
//! MMIO_OUT = 0x102;
//! ```
//!
//! ##### `src/mmio.rs`
//! ```rust
//! unsafe extern "C" {
//!     static MMIO_IN: u16;
//!     static mut MMIO_OUT: u16;
//! }
//!
//! pub fn get_in() -> u16 {
//!     unsafe { core::ptr::read_volatile(&raw MMIO_IN) }
//! }
//! 
//! pub fn set_out(value: u16) {
//!     unsafe { core::ptr::write_volatile(&raw mut MMIO_OUT, value) }
//! }
//! ```

#![no_std]
#![feature(asm_experimental_arch)]
#![feature(doc_auto_cfg)]
#![doc(html_logo_url = "https://aelsi2.github.io/cdm-rs/logo.png")]

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

// Initialization code
#[cfg(not(feature = "harvard"))]
core::arch::global_asm!(
    ".section .text._start",
    ".global _start",
    ".type _start,%function",
    "_start:",
    "ldi fp, __stack_start",
    "stsp fp",
    "jsr main",
    "halt",
);

#[cfg(feature = "harvard")]
core::arch::global_asm!(
    ".section .text._start",
    ".global _start",
    ".type _start,%function",
    "_start:",
    "ldi fp, __stack_start",
    "stsp fp",
    "ldi r0, __data_rom",
    "ldi r1, __data",
    "ldi r2, __data_length",
    "cmp r2, 0",
    "br 1f",
    "0:",
    "lcw r0, r3",
    "stw r1, r3",
    "add r0, 2",
    "add r1, 2",
    "add r2, -2",
    "1:",
    "bnz 0b",
    "ldi r0, __bss",
    "ldi r1, __bss_length",
    "ldi r2, 0",
    "cmp r1, 0",
    "br 1f",
    "0:",
    "stw r0, r2",
    "add r0, 2",
    "add r1, -2",
    "1:",
    "bnz 0b",
    "jsr main",
    "halt",
);

// Default interrupt and exception handler
core::arch::global_asm!(
    ".section .text._DefaultHandler",
    ".global _DefaultHandler",
    ".type _DefaultHandler,%function",
    "_DefaultHandler:",
    "ldi r0, 0xDED0",
    "ldps r1",
    "or r0, r1, r0",
    "pop r1",
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

// Reset vector
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.reset_vector")]
static __RESET_VECTOR: ExceptionVector = ExceptionVector(Reset, Psr::None);

// Harware-defined exception vectors
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.exceptions")]
static __EXCEPTIONS: [ExceptionVector; EXCEPTION_COUNT] = [
    ExceptionVector(UnalignedSP, Psr::ArithNegative), // psr = 1
    ExceptionVector(UnalignedPC, Psr::ArithZero),     // psr = 2
    ExceptionVector(InvalidInst, Psr::ArithNegative.or(Psr::ArithZero)), // psr = 3
    ExceptionVector(DoubleFault, Psr::ArithOverflow), // psr = 4
];

// Application-specific interrupt vectors
#[cfg(not(feature = "interrupts"))]
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.interrupts")]
static __INTERRUPTS: [InterruptVector; INTERRUPT_COUNT] =
    [InterruptVector::DEFAULT; INTERRUPT_COUNT];
