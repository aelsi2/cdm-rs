use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic_halt(_info: &PanicInfo) -> ! {
    crate::execution::halt();
}
