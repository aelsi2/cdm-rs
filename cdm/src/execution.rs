//! Functions for controlling program execution.

use core::arch::asm;

/// Puts the processor into the `HALTED` state.
///
/// The processor stops executing instructions and handling interrupt requests.
/// The only way to make it run again is to perform a hard reset.
#[inline(always)]
pub fn halt() -> ! {
    unsafe { asm!("halt", options(nostack, noreturn)) }
}

/// Performs a soft reset, fetching the reset vector from the IVT.
///
/// # Safety
/// When using `cdm-rt` with the von Neumann architecture, the processor keeps the old 
/// `.data` and `.bss` sections after the soft reset, so calling this function may lead 
/// to undefined behavior.
///
/// This function is safe to call when using `cdm-rt` with the Harvard architecture, because 
/// the aforementioned sections are initialized at startup.
#[inline(always)]
pub unsafe fn reset() -> ! {
    unsafe { asm!("reset", options(nostack, noreturn)) }
}
