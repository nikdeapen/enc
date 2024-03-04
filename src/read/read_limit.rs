use std::io::Read;

/// A `Read` that limits the number of bytes read from another `Read`.
#[derive(Debug)]
pub struct ReadLimit<'a, R> {
    pub read: &'a mut R,
    pub limit: usize,
}

impl<'a, R: Read> Read for ReadLimit<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf: &mut [u8] = if buf.len() > self.limit {
            &mut buf[..self.limit]
        } else {
            buf
        };
        let read: usize = self.read.read(buf)?;
        self.limit -= read;
        Ok(read)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::{Cursor, Read};

    use crate::ReadLimit;

    #[test]
    fn read() -> Result<(), io::Error> {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![0, 1, 2, 3, 4]);
        let mut limit: ReadLimit<Cursor<Vec<u8>>> = ReadLimit {
            read: &mut cursor,
            limit: 4,
        };

        let mut buffer: Vec<u8> = Vec::default();
        let read: usize = limit.read_to_end(&mut buffer)?;
        assert_eq!(read, 4);
        assert_eq!(buffer, vec![0, 1, 2, 3]);

        Ok(())
    }
}
