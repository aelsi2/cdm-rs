//! Functions for controlling program execution.

use core::arch::asm;
use core::sync::atomic::{Ordering, compiler_fence};

/// Stops the clock, putting the processor into the `HALTED` state.
#[inline]
pub fn halt() -> ! {
    // Make sure that all reads/writes complete before halting.
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("halt", options(nostack, nomem, noreturn)) }
}

/// Performs a soft reset, fetching interrupt vector 0.
#[inline]
pub unsafe fn reset() -> ! {
    // Make sure that all reads/writes complete before resetting.
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("reset", options(nostack, nomem, noreturn)) }
}
