use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read};

/// A `Read` implementation that reads an exact number of bytes.
pub struct ReadExact<'a, R> {
    read: &'a mut R,
    remaining: usize,
}

impl<'a, R: Read> ReadExact<'a, R> {
    //! Construction

    /// Creates a `ReadExact` that will read exactly `count` bytes.
    pub fn new(read: &'a mut R, count: usize) -> Self {
        Self {
            read,
            remaining: count,
        }
    }
}

impl<'a, R: Read> Read for ReadExact<'a, R> {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining == 0 {
            Ok(0)
        } else {
            let b: &mut [u8] = if b.len() > self.remaining {
                &mut b[..self.remaining]
            } else {
                b
            };
            match self.read.read(b) {
                Ok(count) => {
                    debug_assert!(count <= self.remaining);
                    if count == 0 {
                        if b.len() == 0 {
                            Ok(0)
                        } else {
                            Err(Error::new(
                                InvalidData,
                                format!("expected {} more bytes", self.remaining),
                            ))
                        }
                    } else {
                        self.remaining -= count;
                        Ok(count)
                    }
                }
                Err(error) => Err(error),
            }
        }
    }
}
