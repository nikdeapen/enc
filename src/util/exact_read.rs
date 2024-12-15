use std::cmp::min;
use std::io::{Error, ErrorKind, Read};

/// A `Read` that reads an exact number of bytes.
#[derive(Debug)]
pub struct ExactRead<'a, R: Read> {
    read: &'a mut R,
    remaining: usize,
}

impl<'a, R: Read> ExactRead<'a, R> {
    //! Construction

    /// Creates a new `ExactRead` with the byte `count`.
    pub fn new(read: &'a mut R, count: usize) -> Self {
        Self {
            read,
            remaining: count,
        }
    }
}

impl<'a, R: Read> Read for ExactRead<'a, R> {
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        let max: usize = min(b.len(), self.remaining);
        let b: &mut [u8] = &mut b[..max];
        self.read.read(b)
    }
}

impl<'a, R: Read> ExactRead<'a, R> {
    /// Finish

    /// Completes the reading.
    ///
    /// Returns an error if the remaining bytes is not `0`.
    pub fn finish(self) -> Result<(), Error> {
        if self.remaining != 0 {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} bytes remaining in ExactRead::finish", self.remaining),
            ))
        } else {
            Ok(())
        }
    }
}
