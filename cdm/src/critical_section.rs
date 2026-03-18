use core::arch::asm;
use critical_section::{Impl, RawRestoreState, set_impl};

struct CDMCriticalSection;
set_impl!(CDMCriticalSection);

unsafe impl Impl for CDMCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_state: RawRestoreState;
        unsafe {
            // Back up the value of PSR
            // and disable interrupts
            asm!(
                "ldps r0", 
                "di",
                out("r0") restore_state,
                options(nostack, nomem),
            );
        }
        restore_state
    }

    unsafe fn release(token: RawRestoreState) {
        unsafe {
            // Resture the value of PSR
            asm!(
                "stps r0",
                in("r0") token,
                options(nostack, nomem),
            );
        }
    }
}
