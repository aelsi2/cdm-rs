use crate::Uart;
use core::fmt::{Error, Result, Write};

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        let bytes = s.as_bytes();
        if bytes.len() == 0 {
            return Ok(());
        }
        for byte in bytes {
            if !self.is_connected() {
                return Err(Error);
            }
            self.write_byte(*byte);
        }
        Ok(())
    }
}
