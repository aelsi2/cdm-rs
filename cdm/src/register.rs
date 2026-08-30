//! Processor register types and functions.

pub mod psr {
    //! Processor status register (PSR).
    use core::arch::asm;

    /// Processor status register bit field.
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
        pub const CONTEXT_BITS: Self = Self::from_bits(0xFF0);

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

        /// Inverts the bits of the processor status value.
        ///
        /// *Note: affects the MMU context bits on `cdm16e`.*
        #[inline(always)]
        pub const fn not(self) -> Self {
            Self(!self.bits())
        }

        /// ANDs the bits of two processor status values.
        ///
        /// *Note: affects the MMU context bits on `cdm16e`.*
        #[inline(always)]
        pub const fn and(self, other: Self) -> Self {
            Self(self.bits() & other.bits())
        }

        /// ORs the bits of two processor status values.
        ///
        /// *Note: affects the MMU context bits on `cdm16e`.*
        #[inline(always)]
        pub const fn or(self, other: Self) -> Self {
            Self(self.bits() | other.bits())
        }

        /// XORs the bits of two processor status values.
        ///
        /// *Note: affects the MMU context bits on `cdm16e`.*
        #[inline(always)]
        pub const fn xor(self, other: Self) -> Self {
            Self(self.bits() ^ other.bits())
        }

        /// Replaces the MMU context number in a processor status value.
        #[doc(cfg(target_feature = "e"))]
        #[cfg(any(target_feature = "e", doc))]
        #[inline(always)]
        pub const fn with_context_number(self, context_number: u8) -> Self {
            Self::from_context_number(context_number).or(self.and(Self::CONTEXT_BITS.not()))
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
            (self.and(Self::CONTEXT_BITS).bits() >> 4) as u8
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

    /// Reads the value of the processor status register (PSR).
    ///
    /// *Note: arithmetic flags are volatile an may change as a side effect of almost any instruction.
    /// If you want to work with these flags, please use inline assembly blocks.*
    #[inline(always)]
    pub fn read() -> Psr {
        let value: u16;
        unsafe { asm!("ldps {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        Psr::from(value)
    }

    /// Writes the value to the processor status register (PSR).
    ///
    /// # Safety
    /// Setting the interrupt enable bit in the PSR will immediately enable interrupts, ending the
    /// current critical section (if there is one), which may lead to a race condition if an interrupt is triggered.
    ///
    /// The values of the reserved bits must be preserved. Failing to do so may lead to undefined behavior
    /// on an extended CPU where these bits are actually used. Therefore, each write to the register should be preceded
    /// with a read to get the values of the reserved bits.
    #[inline(always)]
    pub unsafe fn write(value: Psr) {
        let val: u16 = value.into();
        unsafe { asm!("stps {}", in(reg) val, options(nostack)) }
    }
}

pub mod pc {
    //! Program counter (PC).
    //!
    //! *Note: this module is purely for debugging purposes. If you want to work with this register
    //! directly, please use inline assembly blocks.*

    // Please use the `cdm::register::pc::read!()` alias instead of using this macro directly.
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

    /// Reads the value of the program counter (PC) register.
    ///
    /// # Requirements
    /// The macro expands to an inline assembly block. It requires enabling the `asm_experimental_arch`
    /// compiler feature in your crate. This can be done by adding the following line to the top of
    /// your crate's `main.rs` or `lib.rs`:
    /// ```rust
    /// #![feature(asm_experimental_arch)]
    /// ```
    ///
    /// *Note: this is a macro and not a function, because if it was, it would always return the address of
    /// the function. The function could (and most likely would) be inlined, but this would be unreliable,
    /// because even `#[inline(always)]` does not guarantee this.*
    #[doc(inline)]
    pub use crate::internal_pc_read as read;
}

pub mod sp {
    //! Stack pointer (SP).
    //!
    //! *Note: this module is purely for debugging purposes. If you want to work with this register
    //! directly, please use inline assembly blocks.*

    // Please use the `cdm::register::sp::read!()` alias instead of using this macro directly.
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

    /// Reads the value of the stack pointer (SP) register.
    ///
    /// # Requirements
    /// The macro expands to an inline assembly block. It requires enabling the `asm_experimental_arch`
    /// compiler feature in your crate. This can be done by adding the following line to the top of
    /// your crate's `main.rs` or `lib.rs`:
    /// ```rust
    /// #![feature(asm_experimental_arch)]
    /// ```
    ///
    /// *Note: this is a macro and not a function, because SP is often modified in the function prologue,
    /// so a function may return an inaccurate result, pointing to the end of the next stack frame
    /// instead of the frame of the function where it was used. The function could (and most likely would) be inlined,
    /// but this would be unreliable, because even `#[inline(always)]` does not guarantee this.*
    #[doc(inline)]
    pub use crate::internal_sp_read as read;
}

pub mod fp {
    //! Frame pointer (FP).
    //!
    //! *Note: this module is purely for debugging purposes. If you want to work with this register
    //! directly, please use inline assembly blocks.*

    // Please use the `cdm::register::fp::read!()` alias instead of using this macro directly.
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

    /// Reads the value of the frame pointer (FP) register.
    ///
    /// *Note: the compiler can sometimes omit FP in a function, so FP may contain the address of
    /// the frame from the immediate caller, or some other caller down the stack.*
    ///
    /// # Requirements
    /// The macro expands to an inline assembly block. It requires enabling the `asm_experimental_arch`
    /// compiler feature in your crate. This can be done by adding the following line to the top of
    /// your crate's `main.rs` or `lib.rs`:
    /// ```rust
    /// #![feature(asm_experimental_arch)]
    /// ```
    ///
    /// *Note: this is a macro and not a function, because FP is often modified in the function prologue,
    /// so a function may return an inaccurate result, pointing to the next stack frame instead of
    /// the frame of the function where it was used. The function could (and most likely would) be inlined,
    /// but this would be unreliable, because `#[inline(always)]` does not guarantee this.*
    #[doc(inline)]
    pub use crate::internal_fp_read as read;
}

#[doc(cfg(target_feature = "e"))]
#[cfg(any(target_feature = "e", doc))]
pub mod ssp {
    //! Shadow stack pointer (SSP).
    //!
    //! This register stores the user mode value of SP while executing in system (kernel) mode.
    use core::arch::asm;

    /// Reads the value of the shadow stack pointer (SSP) register.
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe { asm!("ldssp {}", out(reg) value, options(nomem, nostack, preserves_flags)) }
        value
    }

    /// Writes `value` to the shadow stack pointer (SSP) register.
    ///
    /// # Safety
    /// The value stored in the SSP register must be a valid reference to the last word on the user
    /// stack (a multiple of 2 bytes) when returning to user code. Failing to meet this condition
    /// will lead to undefined behavior (likely memory corruption and crash).
    /// Having an invalid intermediate value while executing in system mode is allowed.
    #[inline(always)]
    pub unsafe fn write(value: usize) {
        unsafe { asm!("stssp {}", in(reg) value, options(nomem, nostack, preserves_flags)) }
    }
}
