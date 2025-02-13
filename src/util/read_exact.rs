use std::io::ErrorKind::UnexpectedEof;
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

impl<'a, R: Read> ReadExact<'a, R> {
    //! Properties

    /// Gets the number of remaining bytes.
    pub fn remaining(&self) -> usize {
        self.remaining
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
                                UnexpectedEof,
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

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::ErrorKind::UnexpectedEof;
    use std::io::{Cursor, Read};

    use crate::{read_optional_byte, ReadExact};

    #[test]
    fn read_exact_singles() -> Result<(), Box<dyn Error>> {
        let data: Vec<u8> = vec![0xFF; 3];
        let mut data: Cursor<Vec<u8>> = Cursor::new(data);
        let mut data: ReadExact<Cursor<Vec<u8>>> = ReadExact::new(&mut data, 2);

        assert_eq!(data.remaining, 2);
        match read_optional_byte(&mut data)? {
            Some(b) => assert_eq!(b, 0xFF),
            None => assert!(false),
        }
        assert_eq!(data.remaining, 1);
        match read_optional_byte(&mut data)? {
            Some(b) => assert_eq!(b, 0xFF),
            None => assert!(false),
        }
        assert_eq!(data.remaining, 0);
        match read_optional_byte(&mut data)? {
            Some(b) => assert!(false, "b={}", b),
            None => {}
        }
        assert_eq!(data.remaining, 0);

        Ok(())
    }

    #[test]
    fn read_exact_larger_buffer() -> Result<(), Box<dyn Error>> {
        let data: Vec<u8> = vec![0xFF; 3];
        let mut data: Cursor<Vec<u8>> = Cursor::new(data);
        let mut data: ReadExact<Cursor<Vec<u8>>> = ReadExact::new(&mut data, 2);

        let mut buffer: [u8; 4] = [0u8; 4];
        assert_eq!(data.remaining, 2);
        match data.read(&mut buffer) {
            Ok(count) => {
                assert_eq!(count, 2);
                assert_eq!(buffer[0], 0xFF);
                assert_eq!(buffer[1], 0xFF);
                assert_eq!(buffer[2], 0);
                assert_eq!(buffer[3], 0);
            }
            Err(error) => assert!(false, "{}", error),
        }
        assert_eq!(data.remaining, 0);
        match data.read(&mut buffer) {
            Ok(count) => assert_eq!(count, 0),
            Err(error) => assert!(false, "{}", error),
        }
        assert_eq!(data.remaining, 0);

        Ok(())
    }

    #[test]
    fn read_exact_not_enough() -> Result<(), Box<dyn Error>> {
        let data: Vec<u8> = vec![0xFF; 3];
        let mut data: Cursor<Vec<u8>> = Cursor::new(data);
        let mut data: ReadExact<Cursor<Vec<u8>>> = ReadExact::new(&mut data, 5);

        let mut buffer: [u8; 5] = [0u8; 5];
        assert_eq!(data.remaining(), 5);
        match data.read(&mut buffer) {
            Ok(count) => assert_eq!(count, 3),
            Err(error) => assert!(false, "{}", error),
        }
        assert_eq!(data.remaining(), 2);
        match data.read(&mut buffer[..0]) {
            Ok(count) => assert_eq!(count, 0),
            Err(error) => assert!(false, "{}", error),
        }
        assert_eq!(data.remaining(), 2);
        match data.read(&mut buffer) {
            Ok(count) => assert!(false, "count={}", count),
            Err(error) => assert_eq!(error.kind(), UnexpectedEof),
        }

        Ok(())
    }
}
