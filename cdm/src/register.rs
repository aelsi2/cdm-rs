//! Functions for accessing processor registers.

pub mod psr {
    //! Processor status register (PSR).
    use bitmask_enum::bitmask;
    use core::arch::asm;

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
    #[inline]
    pub fn read() -> Psr {
        let value: u16;
        unsafe {
            asm!("ldps r0", out("r0") value, options(nomem, nostack, preserves_flags));
        }
        Psr::from(value)
    }

    /// Writes `value` to the register.
    #[inline]
    pub unsafe fn write(value: Psr) {
        let val: u16 = value.into();
        unsafe {
            asm!("stps r0", in("r0") val, options(nomem, nostack));
        }
    }
}

pub mod pc {
    //! Program counter (PC).
    use core::arch::asm;

    /// Reads the register value.
    #[inline]
    pub fn read() -> usize {
        let value: usize;
        unsafe {
            asm!("ldpc r0", out("r0") value, options(nomem, nostack, preserves_flags));
        }
        value
    }

    /// Writes `value` to the register.
    #[inline]
    pub unsafe fn write(value: usize) {
        unsafe {
            asm!("stpc r0", in("r0") value);
        }
    }
}

pub mod sp {
    //! Stack pointer (SP).
    use core::arch::asm;

    /// Reads the register value.
    #[inline]
    pub fn read() -> usize {
        let value: usize;
        unsafe {
            asm!("ldsp r0", out("r0") value, options(nomem, nostack, preserves_flags));
        }
        value
    }

    /// Writes `value` to the register.
    #[inline]
    pub unsafe fn write(value: usize) {
        unsafe {
            asm!("stsp r0", in("r0") value, options(nomem, preserves_flags));
        }
    }
}

pub mod fp {
    //! Frame pointer (FP).
    use core::arch::asm;

    /// Reads the register value.
    #[inline]
    pub fn read() -> usize {
        let value: usize;
        unsafe {
            asm!("move fp, r0", out("r0") value, options(nomem, nostack, preserves_flags));
        }
        value
    }

    /// Writes `value` to the register.
    #[inline]
    pub unsafe fn write(value: usize) {
        unsafe {
            asm!("move r0, fp", in("r0") value, options(nomem, preserves_flags));
        }
    }
}
