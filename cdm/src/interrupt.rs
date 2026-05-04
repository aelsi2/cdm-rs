//! Functions for working with interrupts.

use core::arch::asm;

/// Enables all interrupts.
#[inline(always)]
pub unsafe fn enable() {
    unsafe { asm!("ei", options(nostack)) };
}

/// Disables all interrupts.
#[inline(always)]
pub fn disable() {
    unsafe { asm!("di", options(nostack)) };
}

/// Stops the clock until an interrupt request is received,
/// putting the processor into the `WAITING` state.
#[inline(always)]
pub fn wait() {
    unsafe { asm!("wait", options(nostack, preserves_flags)) };
}

/// Triggers a software interrupt with the number `V`.
///
/// `V` must be in the range [0; 63].
#[inline(always)]
pub unsafe fn trigger<const V: u8>() {
    const {
        assert!(V < 64, "Interrupt vector must be in the range [0; 63]");
    }
    unsafe { asm!("int {}", const V) };
}
