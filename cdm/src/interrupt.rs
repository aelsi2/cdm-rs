//! Functions for working with interrupts.

use core::arch::asm;

/// Enables hardware interrupts.
///
/// # Safety
/// Enabling hardware interrupts will end the current critical section (if there is one).
/// This may lead to a race condition if an interrupt is triggered.
#[inline(always)]
pub unsafe fn enable() {
    unsafe { asm!("ei", options(nostack)) };
}

/// Disables hardware interrupts.
#[inline(always)]
pub fn disable() {
    unsafe { asm!("di", options(nostack)) };
}

/// Puts the processor into the `WAITING` state.
///
/// The processor stops executing instructions until an interrupt request is received.
/// If the processor is waiting with hardware interrupts disabled, an IRQ signal will still
/// wake it up, resuming execution after the `wait` instruction without granting the interrupt
/// request.
#[inline(always)]
pub fn wait() {
    unsafe { asm!("wait", options(nostack, preserves_flags)) };
}

/// Triggers a software interrupt with the number `V`.
///
/// `V` must be in the range [0; 63].
///
/// # Safety
/// Triggering interrupt 0 has the same effect as performing a soft reset (except that the previous
/// values of PC and PS are pushed to the stack), which may cause undefined behavior. 
/// See [`cdm::execution::reset`](function@crate::execution::reset).
///
/// *Note: triggering other interrupt vectors should not cause undefined behavior by itself, but may cause 
/// bugs if the handlers are not prepared for this case.*
#[inline(always)]
pub unsafe fn trigger<const V: u8>() {
    const {
        assert!(V < 64, "Interrupt vector must be in the range [0; 63]");
    }
    unsafe { asm!("int {}", const V) };
}
