//! Processor register types and functions.

pub mod psr {
    //! Processor status register (PSR).
    use core::arch::asm;

    /// Processor status register flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct Psr(
        /// The bit representation of the processor status value.
        pub u16,
    );

    impl Psr {
        /// No flags.
        pub const NONE: Self = Self::from_bits(0x0);

        /// Arithmetic negative flag.
        pub const ARITH_NEGATIVE: Self = Self::from_bits(0x1);

        /// Arithmetic zero flag.
        pub const ARITH_ZERO: Self = Self::from_bits(0x2);

        /// Arithmetic overflow flag.
        pub const ARITH_OVERFLOW: Self = Self::from_bits(0x4);

        /// Arithmetic carry flag.
        pub const ARITH_CARRY: Self = Self::from_bits(0x8);

        /// Bit mask for the MMU context number (8 consecutive bits).
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        pub const MMU_CONTEXT: Self = Self::from_bits(0xFF0);

        /// I/O mapping enable flag.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        pub const IO_ENABLE: Self = Self::from_bits(0x2000);

        /// Processor mode flag (system = 0, user = 1).
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        pub const MODE_USER: Self = Self::from_bits(0x4000);

        /// Interrupt enable flag.
        pub const INTERRUPT_ENABLE: Self = Self::from_bits(0x8000);
    }

    impl Psr {
        /// Creates a processor status value from the corresponding bit representation.
        #[inline(always)]
        pub const fn from_bits(bits: u16) -> Self {
            Self(bits)
        }

        /// Gets the bit representation of the processor status value.
        #[inline(always)]
        pub const fn bits(self) -> u16 {
            self.0
        }

        /// Checks if the processor status value has all of the specified flags.
        #[inline(always)]
        pub const fn contains(self, flags: Psr) -> bool {
            self.bits() & flags.bits() == flags.bits()
        }

        /// Checks if the processor status value has any of the specified flags.
        #[inline(always)]
        pub const fn intersects(self, flags: Psr) -> bool {
            self.bits() & flags.bits() != 0
        }

        /// Inverts the flags of the processor status value.
        #[inline(always)]
        pub const fn not(self) -> Self {
            Self(!self.bits())
        }

        /// ANDs the flags of two processor status values.
        #[inline(always)]
        pub const fn and(self, other: Self) -> Self {
            Self(self.bits() & other.bits())
        }

        /// ORs the flags of two processor status values.
        #[inline(always)]
        pub const fn or(self, other: Self) -> Self {
            Self(self.bits() | other.bits())
        }

        /// XORs the flags of two processor status values.
        #[inline(always)]
        pub const fn xor(self, other: Self) -> Self {
            Self(self.bits() ^ other.bits())
        }

        /// Creates a processor status value from a MMU context number.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        #[inline(always)]
        pub const fn from_context_number(context_number: u8) -> Self {
            Self::from_bits((context_number as u16) << 4)
        }

        /// Extracts the MMU context number from the processor status value.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        #[inline(always)]
        pub const fn context_number(self) -> u8 {
            (self.and(Self::MMU_CONTEXT).bits() >> 4) as u8
        }
    }

    impl From<u16> for Psr {
        fn from(bits: u16) -> Self {
            Self::from_bits(bits)
        }
    }

    impl From<Psr> for u16 {
        fn from(value: Psr) -> Self {
            value.bits()
        }
    }

    impl core::ops::Not for Psr {
        type Output = Psr;

        fn not(self) -> Self::Output {
            self.not()
        }
    }

    impl core::ops::BitAnd for Psr {
        type Output = Psr;

        fn bitand(self, rhs: Self) -> Self::Output {
            self.and(rhs)
        }
    }

    impl core::ops::BitAndAssign for Psr {
        fn bitand_assign(&mut self, rhs: Self) {
            *self = self.and(rhs);
        }
    }

    impl core::ops::BitOr for Psr {
        type Output = Psr;

        fn bitor(self, rhs: Self) -> Self::Output {
            self.or(rhs)
        }
    }

    impl core::ops::BitOrAssign for Psr {
        fn bitor_assign(&mut self, rhs: Self) {
            *self = self.or(rhs);
        }
    }

    impl core::ops::BitXor for Psr {
        type Output = Psr;

        fn bitxor(self, rhs: Self) -> Self::Output {
            self.xor(rhs)
        }
    }

    impl core::ops::BitXorAssign for Psr {
        fn bitxor_assign(&mut self, rhs: Self) {
            *self = self.xor(rhs);
        }
    }

    impl core::fmt::Binary for Psr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Binary::fmt(&self.0, f)
        }
    }

    impl core::fmt::LowerHex for Psr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::LowerHex::fmt(&self.0, f)
        }
    }

    impl core::fmt::UpperHex for Psr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::UpperHex::fmt(&self.0, f)
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
