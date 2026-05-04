//! Functions for controlling program execution.

use core::arch::asm;

/// Stops the clock, putting the processor into the `HALTED` state.
#[inline(always)]
pub fn halt() -> ! {
    unsafe { asm!("halt", options(nostack, noreturn)) }
}

/// Performs a soft reset, fetching interrupt vector 0.
#[inline(always)]
pub unsafe fn reset() -> ! {
    unsafe { asm!("reset", options(nostack, noreturn)) }
}
