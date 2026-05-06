use crate::{Uart, UartError};
use embedded_io::{Error, ErrorKind, ErrorType, Read, ReadReady, Write, WriteReady};

impl ErrorType for Uart {
    type Error = UartError;
}

impl Error for UartError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::NotConnected => ErrorKind::BrokenPipe,
        }
    }
}

impl Write for Uart {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.len() == 0 {
            return Ok(0);
        }
        let mut written: usize = 0;
        if !self.is_connected() {
            return Err(UartError::NotConnected);
        }
        for c in buf {
            self.write_byte(*c);
            written += 1;
            if !self.is_connected() {
                break;
            }
        }
        Ok(written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Read for Uart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if !self.is_connected() {
            return Ok(0);
        }
        let mut read: usize = 0;
        for i in 0..buf.len() {
            while !self.has_data() {}
            buf[i] = self.read_byte();
            read += 1;
            if !self.has_data() {
                break;
            }
        }
        Ok(read)
    }
}

impl WriteReady for Uart {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_connected())
    }
}

impl ReadReady for Uart {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.has_data())
    }
}
