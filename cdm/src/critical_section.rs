use crate::interrupt;
use crate::register::psr;
use critical_section::{Impl, RawRestoreState, set_impl};

struct CDMCriticalSection;
set_impl!(CDMCriticalSection);

unsafe impl Impl for CDMCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore_state: RawRestoreState = psr::read().into();
        interrupt::disable();
        restore_state
    }

    unsafe fn release(token: RawRestoreState) {
        unsafe { psr::write(token.into()) }
    }
}
