//! Functions for controlling program execution.

use core::arch::asm;

/// Halts the processor.
#[inline]
pub fn halt() -> ! {
    unsafe {
        asm!("halt", options(noreturn));
    }
}

/// Performs a soft reset.
#[inline]
pub unsafe fn reset() -> ! {
    unsafe {
        asm!("reset", options(noreturn));
    }
}
