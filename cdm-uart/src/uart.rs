/// Logisim UART driver
///
/// The driver is stateless, so instances can be constructed and dropped as needed.
/// Despite this, you need to be careful when using UART in interrupt handlers, as the driver doesn't use any
/// exclusion mechanisms, so multi-byte reads and writes may get corrupted.
pub struct Uart {
    flags: *const u8,
    data: *mut u8,
}

/// Logisim UART error
#[derive(Debug)]
pub enum UartError {
    /// The UART component is not connected to a client.
    NotConnected,
}

impl Uart {
    /// Constructs a new instance of [`Uart`].
    ///
    /// `flags` should point to a 8-bit-wide read-only MMIO register.
    /// The value in the register is a bit mask:
    /// - bit 0 (LSB) - there is an active connection.
    /// - bit 1 - there is data in the read bufer.
    /// - bits 2-7 - reserved.
    ///
    /// `data` should point to a 8-bit-wide read-write MMIO register.
    /// Writing to this register should add a byte to the end of the write queue.
    /// Reading from this register should get and remove a byte from the start of read queue.
    pub const unsafe fn new(flags: *const u8, data: *mut u8) -> Self {
        Self { flags, data }
    }

    /// Checks if the UART component has an active connection.
    pub fn is_connected(&self) -> bool {
        const UART_CONNECTED: u8 = 1;
        (self.flags() & UART_CONNECTED) != 0
    }

    /// Checks if the UART component has at least one byte in the read queue.
    pub fn has_data(&self) -> bool {
        const UART_HAS_DATA: u8 = 2;
        self.flags() & UART_HAS_DATA != 0
    }

    /// Removes a byte from the read queue.
    /// Returns the removed byte or an unspecified value if the queue was empty.
    pub fn read_byte(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.data) }
    }

    /// Puts a byte into the write queue, if there is an active connection.
    ///
    /// Note that due to limitations of the Logisim UART component, there is no way to know if the
    /// write was successful. The only thing that can be done to detect errors is checking the
    /// connection state every time before peforming a write, but the state can change on any clock cycle, so this
    /// approach is unreliable.
    pub fn write_byte(&mut self, byte: u8) {
        unsafe { core::ptr::write_volatile(self.data, byte) }
    }

    fn flags(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.flags) }
    }
}

impl core::fmt::Display for UartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotConnected => {
                write!(f, "no active UART connection")
            }
        }
    }
}

impl core::error::Error for UartError {}
