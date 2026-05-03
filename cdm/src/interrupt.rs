//! Functions for working with interrupts.

use core::arch::asm;
use core::sync::atomic::{Ordering, compiler_fence};

/// Enables all interrupts.
#[inline]
pub unsafe fn enable() {
    // Make sure that all reads/writes before ei stay in the critical section.
    compiler_fence(Ordering::SeqCst);
    unsafe { asm!("ei", options(nostack, nomem)) };
}

/// Disables all interrupts.
#[inline]
pub fn disable() {
    unsafe { asm!("di", options(nostack, nomem)) };
    // Make sure that all reads/writes after di stay in the critical section.
    compiler_fence(Ordering::SeqCst);
}

/// Stops the clock until an interrupt request is received,
/// putting the processor into the `WAITING` state.
#[inline]
pub fn wait() {
    // Make sure that all memory accesses above this are done before the wait.
    compiler_fence(Ordering::SeqCst);
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
    unsafe { asm!("int {}", const V) };
}
