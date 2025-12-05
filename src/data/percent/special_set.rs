use std::fmt::{Display, Formatter};

/// A set of special US-ASCII punctuation chars for percent encoding.
///
/// This set is designed to fit into a single `u32` value so that it can be `Copy`.
///
/// Note:
/// - The `%` char is not included since it must always be encoded in any percent-encoded string.
/// - The `SPACE` char is included, although it is not defined as a punctuation char.
///
/// # Chars
/// The 32 valid chars that can be present in the set are:
/// 1:  SPACE
/// 4:  !"#$
/// 10: &'()*+,-./
/// 7:  :;<=>?@
/// 6:  [\]^_`
/// 4:  {|}~
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct SpecialSet {
    bits: u32,
}

impl From<&str> for SpecialSet {
    fn from(chars: &str) -> Self {
        let mut set: Self = Self::default();
        for c in chars.as_bytes() {
            set.add(*c);
        }
        set
    }
}

impl SpecialSet {
    //! Constants

    /// The set of valid chars. (in order of both their bit index and their ASCII code)
    const CHARS: &'static [u8; 32] = b" !\"#$&'()*+,-./:;<=>?@[\\]^_`{|}~";

    /// The index table.
    const INDEX_TABLE: [u8; 128] = Self::create_index_table();

    /// Creates the index table.
    const fn create_index_table() -> [u8; 128] {
        let mut index_table: [u8; 128] = [32; 128];

        index_table[b' ' as usize] = 0;
        index_table[b'!' as usize] = 1;
        index_table[b'"' as usize] = 2;
        index_table[b'#' as usize] = 3;
        index_table[b'$' as usize] = 4;
        index_table[b'&' as usize] = 5;
        index_table[b'\'' as usize] = 6;
        index_table[b'(' as usize] = 7;
        index_table[b')' as usize] = 8;
        index_table[b'*' as usize] = 9;
        index_table[b'+' as usize] = 10;
        index_table[b',' as usize] = 11;
        index_table[b'-' as usize] = 12;
        index_table[b'.' as usize] = 13;
        index_table[b'/' as usize] = 14;
        index_table[b':' as usize] = 15;
        index_table[b';' as usize] = 16;
        index_table[b'<' as usize] = 17;
        index_table[b'=' as usize] = 18;
        index_table[b'>' as usize] = 19;
        index_table[b'?' as usize] = 20;
        index_table[b'@' as usize] = 21;
        index_table[b'[' as usize] = 22;
        index_table[b'\\' as usize] = 23;
        index_table[b']' as usize] = 24;
        index_table[b'^' as usize] = 25;
        index_table[b'_' as usize] = 26;
        index_table[b'`' as usize] = 27;
        index_table[b'{' as usize] = 28;
        index_table[b'|' as usize] = 29;
        index_table[b'}' as usize] = 30;
        index_table[b'~' as usize] = 31;

        index_table
    }
}

impl SpecialSet {
    //! Index

    /// Gets the bit index for `c`.
    ///
    /// Returns `32` for invalid chars.
    #[inline(always)]
    const fn index_of(c: u8) -> usize {
        if c > 0x7F {
            32
        } else {
            Self::INDEX_TABLE[(c as usize) & 0x7F] as usize
        }
    }
}

impl SpecialSet {
    //! Mutations

    /// Adds `c`. If `c` is invalid or already present, this has no effect.
    pub fn add(&mut self, c: u8) {
        self.bits |= 1u32.checked_shl(Self::index_of(c) as u32).unwrap_or(0)
    }

    /// Removes `c`. If `c` is invalid or not present, this has no effect.
    pub fn remove(&mut self, c: u8) {
        self.bits &= !1u32.checked_shl(Self::index_of(c) as u32).unwrap_or(0);
    }
}

impl SpecialSet {
    //! Properties

    /// Checks if the set contains `c`.
    ///
    /// If `c` is invalid, this will return false.
    pub fn contains(&self, c: u8) -> bool {
        (self.bits & 1u32.checked_shl(Self::index_of(c) as u32).unwrap_or(0)) != 0
    }

    /// Gets the number of chars in the set.
    pub fn size(&self) -> usize {
        self.bits.count_ones() as usize
    }
}

impl SpecialSet {
    //! Iteration

    /// Creates a new iterator for the chars.
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        SpecialSetIterator { bits: self.bits }
    }
}

struct SpecialSetIterator {
    bits: u32,
}

impl Iterator for SpecialSetIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let index: u32 = self.bits.trailing_zeros();
            self.bits &= !(1u32 << index);
            Some(SpecialSet::CHARS[index as usize])
        }
    }
}

impl Display for SpecialSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.iter() {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::percent::special_set::SpecialSet;

    #[test]
    fn add_contains_remove_size() {
        let mut set: SpecialSet = SpecialSet::default();

        for (i, c) in SpecialSet::CHARS.iter().enumerate() {
            assert_eq!(i, set.size());
            assert!(!set.contains(*c));
            set.add(*c);
            assert!(set.contains(*c));
            assert_eq!(i + 1, set.size());
        }

        for (i, c) in SpecialSet::CHARS.iter().enumerate() {
            assert_eq!(32 - i, set.size());
            assert!(set.contains(*c));
            set.remove(*c);
            assert!(!set.contains(*c));
            assert_eq!(32 - (i + 1), set.size());
        }
    }

    #[test]
    fn invalid_chars() {
        let mut set: SpecialSet = SpecialSet::default();

        set.add(b'\x00');
        assert_eq!(set.size(), 0);

        set.remove(b'\x00');
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn iter_display() {
        let mut set: SpecialSet = SpecialSet::default();

        for c in SpecialSet::CHARS.iter() {
            set.add(*c);
        }

        assert_eq!(set.to_string(), " !\"#$&'()*+,-./:;<=>?@[\\]^_`{|}~");
    }
}
