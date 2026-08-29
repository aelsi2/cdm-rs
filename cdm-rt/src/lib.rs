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

/// The number of exception vectors in the interrupt vector table, including the start vector.
pub const EXCEPTION_COUNT: usize = 16;

/// The number of application-specific interrupt vectors in the interrupt vector table.
pub const INTERRUPT_COUNT: usize = 64 - EXCEPTION_COUNT;

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
    /// Calls `_interrupt_handler`, which triggers a hardware halt by default.
    pub const DEFAULT: Self = {
        unsafe extern "cdm-isr" {
            fn _interrupt_handler();
        }
        InterruptVector(_interrupt_handler, Psr::NONE)
    };
}

/// Defines the application-specific interrupt handler section of the interrupt vector table.
///
/// The interrupt vectors specified in the arguments are placed sequentially after exception vectors,
/// starting from index `EXCEPTION_COUNT`. The rest of the table is filled with `InterruptVector::DEFAULT`.
///
/// Must be used **once** in the dependency graph.
///
/// Use the [`interrupt`](macro@interrupt) attribute to define interrupt handler functions.
///
/// ``` no_run
/// interrupt_vectors![
///     InterruptVector(MyHandler1, Psr::NONE), // int EXCEPTION_COUNT+0
///     InterruptVector(MyHandler2, Psr::NONE), // int EXCEPTION_COUNT+1
///     InterruptVector(MyHandler3, Psr::NONE), // int EXCEPTION_COUNT+2
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
    "addsp -8",
    "jsr main",
    "addsp 8",
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
    "addsp -8",
    "jsr main",
    "addsp 8",
    "halt",
);

// Default interrupt and exception handler
core::arch::global_asm!(
    ".section .text._default_handler",
    ".global _default_handler",
    ".type _default_handler,%function",
    "_default_handler:",
    "ldi r0, 0xDED0",
    "ldps r1",
    "or r0, r1, r0",
    "pop r1",
    "halt",
);

unsafe extern "C" {
    #[link_name = "_start"]
    fn reset() -> !;
    #[link_name = "_ex_unaligned_sp"]
    fn unaligned_sp() -> !;
    #[link_name = "_ex_unaligned_pc"]
    fn unaligned_pc() -> !;
    #[link_name = "_ex_invalid_inst"]
    fn invalid_inst() -> !;
    #[link_name = "_ex_double_fault"]
    fn double_fault() -> !;
    #[link_name = "_ex_priv_violation"]
    fn priv_violation() -> !;
    #[link_name = "_ex_reserved_6"]
    fn ex_reserved_6() -> !;
    #[link_name = "_ex_system_call"]
    fn system_call() -> !;
    #[link_name = "_ex_reserved_8"]
    fn ex_reserved_8() -> !;
    #[link_name = "_ex_reserved_9"]
    fn ex_reserved_9() -> !;
    #[link_name = "_ex_reserved_a"]
    fn ex_reserved_a() -> !;
    #[link_name = "_ex_reserved_b"]
    fn ex_reserved_b() -> !;
    #[link_name = "_ex_reserved_c"]
    fn ex_reserved_c() -> !;
    #[link_name = "_ex_reserved_d"]
    fn ex_reserved_d() -> !;
    #[link_name = "_ex_reserved_e"]
    fn ex_reserved_e() -> !;
    #[link_name = "_ex_reserved_f"]
    fn ex_reserved_f() -> !;
}

// Harware-defined exception vectors
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.exceptions")]
static __EXCEPTIONS: [ExceptionVector; EXCEPTION_COUNT] = [
    ExceptionVector(reset, Psr::from_bits(0x0)),
    ExceptionVector(unaligned_sp, Psr::from_bits(0x1)),
    ExceptionVector(unaligned_pc, Psr::from_bits(0x2)),
    ExceptionVector(invalid_inst, Psr::from_bits(0x3)),
    ExceptionVector(double_fault, Psr::from_bits(0x4)),
    ExceptionVector(priv_violation, Psr::from_bits(0x5)),
    ExceptionVector(ex_reserved_6, Psr::from_bits(0x6)),
    ExceptionVector(system_call, Psr::from_bits(0x7)),
    ExceptionVector(ex_reserved_8, Psr::from_bits(0x8)),
    ExceptionVector(ex_reserved_9, Psr::from_bits(0x9)),
    ExceptionVector(ex_reserved_a, Psr::from_bits(0xA)),
    ExceptionVector(ex_reserved_b, Psr::from_bits(0xB)),
    ExceptionVector(ex_reserved_c, Psr::from_bits(0xC)),
    ExceptionVector(ex_reserved_d, Psr::from_bits(0xD)),
    ExceptionVector(ex_reserved_e, Psr::from_bits(0xE)),
    ExceptionVector(ex_reserved_f, Psr::from_bits(0xF)),
];

// Application-specific interrupt vectors
#[cfg(not(feature = "interrupts"))]
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.interrupts")]
static __INTERRUPTS: [InterruptVector; INTERRUPT_COUNT] =
    [InterruptVector::DEFAULT; INTERRUPT_COUNT];
