//! Functions for accessing processor registers.

pub mod psr {
    //! Processor status register (PSR).
    use bitmask_enum::bitmask;
    use core::arch::asm;
    use core::sync::atomic::{Ordering, compiler_fence};

    /// Processor status register flags.
    #[bitmask(u16)]
    pub enum Psr {
        /// No flags.
        None = 0x0,
        /// Arithmetic negative flag.
        ArithNegative = 0x1,
        /// Arithmetic zero flag.
        ArithZero = 0x2,
        /// Arithmetic overflow flag.
        ArithOverflow = 0x4,
        /// Arithmetic carry flag.
        ArithCarry = 0x8,
        /// Interrupt enable flag.
        Interrupt = 0x8000,
    }

    /// Reads the register value.
    #[inline(always)]
    pub fn read() -> Psr {
        let value: u16;
        unsafe { asm!("ldps {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        Psr::from(value)
    }

    /// Writes `value` to the register.
    #[inline(always)]
    pub unsafe fn write(value: Psr) {
        let val: u16 = value.into();
        compiler_fence(Ordering::SeqCst);
        unsafe { asm!("stps {}", in(reg) val, options(nomem, nostack)) }
        compiler_fence(Ordering::SeqCst);
    }
}

pub mod pc {
    //! Program counter (PC).
    use core::arch::asm;

    /// Reads the register value.
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe { asm!("ldpc {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        value
    }
}

pub mod sp {
    //! Stack pointer (SP).
    use core::arch::asm;

    /// Reads the register value.
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe { asm!("ldsp {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        value
    }
}

pub mod fp {
    //! Frame pointer (FP).
    use core::arch::asm;

    /// Reads the register value.
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe { asm!("move fp, {}", out(reg) value, options(nomem, nostack)) }
        value
    }
}
