use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read};

/// A `Read` that produces an exact number of bytes.
pub struct ExactRead<'a, R: Read> {
    read: &'a mut R,
    remaining: usize,
}

impl<'a, R: Read> ExactRead<'a, R> {
    //! Construction

    /// Creates a new `ReadExact`.
    pub fn new(read: &'a mut R, count: usize) -> Self {
        Self {
            read,
            remaining: count,
        }
    }
}

impl<'a, R: Read> Read for ExactRead<'a, R> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining == 0 {
            Ok(0)
        } else {
            let buffer: &mut [u8] = &mut buffer[..self.remaining];
            match self.read.read(buffer) {
                Ok(read) => {
                    if read == 0 {
                        Err(Error::new(InvalidData, "not enough bytes from source read"))
                    } else {
                        self.remaining -= read;
                        Ok(read)
                    }
                }
                Err(error) => Err(error),
            }
        }
    }
}
