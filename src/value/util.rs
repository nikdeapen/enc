use std::io;
use std::io::ErrorKind::UnexpectedEof;

/// Reads a single byte from the `Read`.
#[inline(always)]
pub(crate) fn read_single_byte<R>(r: &mut R) -> Result<u8, io::Error>
where
    R: io::Read,
{
    if let Some(b) = read_optional_byte(r)? {
        Ok(b)
    } else {
        Err(io::Error::new(UnexpectedEof, "no bytes to read"))
    }
}

/// Reads an optional byte from the `Read`.
#[inline(always)]
pub(crate) fn read_optional_byte<R>(r: &mut R) -> Result<Option<u8>, io::Error>
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
