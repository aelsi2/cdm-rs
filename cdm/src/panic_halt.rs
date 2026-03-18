use core::arch::asm;
use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic_halt(_info: &PanicInfo) -> ! {
    unsafe { asm!("halt", options(nomem, nostack, noreturn)) };
}
