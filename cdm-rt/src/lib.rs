#![no_std]
#![feature(asm_experimental_arch)]

pub const EXCEPTION_COUNT: usize = 4;
pub const INTERRUPT_COUNT: usize = 59;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionVector(unsafe extern "C" fn() -> !, u16);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptVector(unsafe extern "cdm-isr" fn(), u16);

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

                unsafe extern "cdm-isr" {
                    fn InterruptHandler();
                }
                let mut arr = [$crate::InterruptVector(InterruptHandler, 0); N];
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
static __INTERRUPTS: [InterruptVector; INTERRUPT_COUNT] = [{
    unsafe extern "cdm-isr" {
        fn InterruptHandler();
    }
    InterruptVector(InterruptHandler, 0)
}; INTERRUPT_COUNT];
