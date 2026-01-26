use crate::Error;
use crate::Error::IntegerOverflow;

/// Gets the encoded length of the optionally `padded` `data_len`.
#[inline(always)]
pub fn encoded_len(data_len: usize, padded: bool) -> Result<usize, Error> {
    let div: usize = data_len / 3;
    let rem: usize = data_len % 3;
    let extra: usize = match rem {
        0 => 0,
        1 => {
            if padded {
                4
            } else {
                2
            }
        }
        2 => {
            if padded {
                4
            } else {
                3
            }
        }
        _ => unreachable!(),
    };
    div.checked_mul(4)
        .ok_or(IntegerOverflow)?
        .checked_add(extra)
        .ok_or(IntegerOverflow)
}

// todo -- test cases
