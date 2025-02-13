use std::io;
use std::io::ErrorKind::UnexpectedEof;

use crate::Error::InvalidEncodedData;

/// Reads a single byte from the `Read`.
#[inline(always)]
pub fn read_single_byte<R>(r: &mut R) -> Result<u8, io::Error>
where
    R: io::Read,
{
    if let Some(b) = read_optional_byte(r)? {
        Ok(b)
    } else {
        Err(io::Error::new(UnexpectedEof, InvalidEncodedData))
    }
}

/// Reads an optional byte from the `Read`.
///
/// Returns `None` when the `Read` has no more data.
#[inline(always)]
pub fn read_optional_byte<R>(r: &mut R) -> Result<Option<u8>, io::Error>
where
    R: io::Read,
{
    let mut buffer: [u8; 1] = [0];
    match r.read(&mut buffer)? {
        0 => Ok(None),
        1 => Ok(Some(buffer[0])),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::{Cursor, ErrorKind};

    use crate::{read_optional_byte, read_single_byte};

    #[test]
    fn fn_read_single_byte() {
        let data: Vec<u8> = vec![0xFF; 1];
        let mut data: Cursor<Vec<u8>> = Cursor::new(data);

        match read_single_byte(&mut data) {
            Ok(b) => assert_eq!(b, 0xFF),
            Err(error) => assert!(false, "{:#?}", error),
        }

        match read_single_byte(&mut data) {
            Ok(b) => assert!(false, "b={}", b),
            Err(error) => assert_eq!(error.kind(), ErrorKind::UnexpectedEof),
        }
    }

    #[test]
    fn fn_read_optional_byte() -> Result<(), Box<dyn Error>> {
        let data: Vec<u8> = vec![0xFF; 1];
        let mut data: Cursor<Vec<u8>> = Cursor::new(data);

        match read_optional_byte(&mut data)? {
            Some(b) => assert_eq!(b, 0xFF),
            None => assert!(false),
        }

        match read_optional_byte(&mut data)? {
            Some(b) => assert!(false, "b={}", b),
            None => {}
        }

        Ok(())
    }
}
