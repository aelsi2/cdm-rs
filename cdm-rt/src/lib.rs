#![no_std]
#![feature(asm_experimental_arch)]

/// The number of exception vectors in the interrupt vector table.
pub const EXCEPTION_COUNT: usize = 4;

/// The index of the first application-specific interrupt vector.
pub const INTERRUPT_START: usize = EXCEPTION_COUNT + 1;

/// The number of application-specific interrupt vectors in the interrupt vector table.
pub const INTERRUPT_COUNT: usize = 59;

/// Represents a vector in the interrupt vector table.
///
/// The first field is the pointer to the handler function.
/// The second field is the initial value of the status register (usually 0 to disable interrupts).
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptVector(pub unsafe extern "cdm-isr" fn(), pub u16);

impl InterruptVector {
    /// The default vector used in absence of an explicit definition.
    ///
    /// Calls `InterruptHandler`, which triggers a hardware halt by default.
    pub const DEFAULT: Self = {
        unsafe extern "cdm-isr" {
            fn InterruptHandler();
        }
        InterruptVector(InterruptHandler, 0)
    };
}

/// Defines the application-specific interrupt handler section of the interrupt vector table.
///
/// The interrupt vectors specified in the arguments are placed sequentially after the reset and
/// exception vectors, starting from index `INTERRUPT_START`. The rest of the table is filled
/// with `InterruptVector::DEFAULT`.
///
/// ```
/// interrupt_vectors![
///     InterruptVector(MyHandler1, 0), // int INTERRUPT_START+0
///     InterruptVector(MyHandler2, 0), // int INTERRUPT_START+1
///     InterruptVector(MyHandler3, 0), // int INTERRUPT_START+2
/// ];
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
struct ExceptionVector(pub unsafe extern "C" fn() -> !, pub u16);

// The initialization code
core::arch::global_asm!(
    ".section .text._start",
    ".global _start",
    ".type _start,%function",
    "_start:",
    "ldi r0, 0",
    "stsp r0",
    "move r0, fp",
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
static __RESET_VECTOR: ExceptionVector = ExceptionVector(Reset, 0);

// Harware-defined exception vectors
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.exceptions")]
static __EXCEPTIONS: [ExceptionVector; EXCEPTION_COUNT] = [
    ExceptionVector(UnalignedSP, 0),
    ExceptionVector(UnalignedPC, 0),
    ExceptionVector(InvalidInst, 0),
    ExceptionVector(DoubleFault, 0),
];

// Application-specific interrupt vectors
#[cfg(not(feature = "interrupts"))]
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".ivt.interrupts")]
static __INTERRUPTS: [InterruptVector; INTERRUPT_COUNT] =
    [InterruptVector::DEFAULT; INTERRUPT_COUNT];
