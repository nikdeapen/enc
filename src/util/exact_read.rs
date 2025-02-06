use std::cmp::min;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{Error, Read, Result};

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

impl<'a, R: Read> ExactRead<'a, R> {
    //! Properties

    /// Gets the number of remaining bytes.
    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<'a, R: Read> Read for ExactRead<'a, R> {
    fn read(&mut self, b: &mut [u8]) -> Result<usize> {
        let max: usize = min(b.len(), self.remaining);
        if max == 0 {
            return Ok(0);
        }
        let b: &mut [u8] = &mut b[..max];
        match self.read.read(b) {
            Ok(read) => {
                if read == 0 {
                    Err(Error::new(
                        UnexpectedEof,
                        format!("{} more bytes expected", self.remaining),
                    ))
                } else {
                    self.remaining -= read;
                    Ok(read)
                }
            }
            Err(error) => Err(error),
        }
    }
}
