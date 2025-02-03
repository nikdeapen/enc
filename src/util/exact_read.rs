use std::cmp::min;
use std::io::Read;

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
    fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
        let max: usize = min(b.len(), self.remaining);
        let b: &mut [u8] = &mut b[..max];
        self.read.read(b)
    }
}
