//! Functions for controlling program execution.

use core::arch::asm;

/// Stops the clock, putting the processor into the `HALTED` state.
#[inline]
pub fn halt() -> ! {
    unsafe {
        asm!("halt", options(noreturn));
    }
}

/// Performs a soft reset, fetching interrupt vector 0.
#[inline]
pub unsafe fn reset() -> ! {
    unsafe {
        asm!("reset", options(noreturn));
    }
}
