//! Functions for working with interrupts.

use core::arch::asm;

/// Enables all interrupts.
#[inline]
pub unsafe fn enable() {
    unsafe { asm!("ei", options(nostack, nomem)) };
}

/// Disables all interrupts.
#[inline]
pub fn disable() {
    unsafe { asm!("di", options(nostack, nomem)) };
}

/// Stops the clock until an interrupt request is received,
/// putting the processor into the `WAITING` state.
#[inline]
pub fn wait() {
    unsafe { asm!("wait", options(nostack, nomem, preserves_flags)) };
}

/// Triggers a software interrupt with the number `V`.
///
/// `V` must be in the range [0; 63].
#[inline]
pub unsafe fn trigger<const V: u8>() {
    const {
        assert!(V < 64, "Interrupt vector must be in the range [0; 63]");
    }
    unsafe { asm!("int {}", const V, options(preserves_flags)) };
}
