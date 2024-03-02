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
        self.read.read(buf)
    }
}
