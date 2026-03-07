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

#[cfg(test)]
mod tests {
    use crate::base_64::encode::encoded_len::encoded_len;

    #[test]
    fn fn_encoded_len() {
        let test_cases: &[(usize, bool, usize)] = &[
            (0, false, 0),
            (0, true, 0),
            (1, false, 2),
            (1, true, 4),
            (2, false, 3),
            (2, true, 4),
            (3, false, 4),
            (3, true, 4),
            (4, false, 6),
            (4, true, 8),
            (5, false, 7),
            (5, true, 8),
            (6, false, 8),
            (6, true, 8),
        ];
        for (data_len, padded, expected) in test_cases {
            let result: usize = encoded_len(*data_len, *padded).unwrap();
            assert_eq!(
                result, *expected,
                "data_len={} padded={}",
                *data_len, *padded
            );
        }
    }
}
