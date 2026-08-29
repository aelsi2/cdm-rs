//! Processor register types and functions.

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
        /// Bit mask for the MMU context number.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        MmuContext = 0xFF0,
        /// I/O mapping enable flag.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        IoEnable = 0x2000,
        /// Processor mode flag (system = 0, user = 1).
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        Mode = 0x4000,
        /// Interrupt enable flag.
        Interrupt = 0x8000,
    }

    impl Psr {
        /// Creates a processor status value from the corresponding bit representation.
        pub const fn from_bits(bits: u16) -> Self {
            Self { bits }
        }

        /// Creates a processor status value from a MMU context number.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        pub const fn from_context_number(context_number: u8) -> Self {
            Self {
                bits: (context_number as u16) << 4,
            }
        }

        /// Extracts the MMU context number from this processor status value.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        #[inline(always)]
        pub const fn context_number(self) -> u8 {
            (self.and(Self::MmuContext).bits() >> 4) as u8
        }
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
        unsafe { asm!("stps {}", in(reg) val, options(nostack)) }
    }
}

pub mod pc {
    //! Program counter (PC).

    #[doc(hidden)]
    #[macro_export]
    macro_rules! internal_pc_read {
        () => {
            {
                let value: usize;
                unsafe { ::core::arch::asm!("ldpc {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
                value
            }
        };
    }

    /// Reads the register value.
    #[doc(inline)]
    pub use crate::internal_pc_read as read;
}

pub mod sp {
    //! Stack pointer (SP).

    #[doc(hidden)]
    #[macro_export]
    macro_rules! internal_sp_read {
        () => {
            {
                let value: usize;
                unsafe { ::core::arch::asm!("ldsp {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
                value
            }
        };
    }

    /// Reads the register value.
    #[doc(inline)]
    pub use crate::internal_sp_read as read;
}

pub mod fp {
    //! Frame pointer (FP).

    #[doc(hidden)]
    #[macro_export]
    macro_rules! internal_fp_read {
        () => {
            {
                let value: usize;
                unsafe { ::core::arch::asm!("move fp, {}", out(reg) value, options(nomem, nostack)) }
                value
            }
        };
    }

    /// Reads the register value.
    #[doc(inline)]
    pub use crate::internal_fp_read as read;
}

#[doc(cfg(target_feature = "e"))]
#[cfg(any(target_feature = "e", doc))]
pub mod ssp {
    //! Shadow stack pointer (SSP).
    use core::arch::asm;

    /// Reads the register value.
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe { asm!("ldssp {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        value
    }

    /// Writes `value` to the register.
    #[inline(always)]
    pub unsafe fn write(value: usize) {
        unsafe { asm!("stssp {}", in(reg) value, options(nomem, nostack, preserves_flags)) }
    }
}
