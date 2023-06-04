use crate::percent::SpecialSet;

#[test]
fn index_of() {
    for (i, c) in SpecialSet::CHARS.iter().enumerate() {
        let index: usize = SpecialSet::index_of(*c).unwrap();
        assert_eq!(index, i);
    }
    assert_eq!(SpecialSet::index_of(b'\x00'), None);
}

#[test]
fn properties() {
    let first_half: &[u8] = &SpecialSet::CHARS[0..16];
    let second_half: &[u8] = &SpecialSet::CHARS[16..32];
    let mut first_set: SpecialSet =
        SpecialSet::from(unsafe { std::str::from_utf8_unchecked(first_half) });
    let mut second_set: SpecialSet =
        SpecialSet::from(unsafe { std::str::from_utf8_unchecked(second_half) });

    assert_eq!(first_set.size(), 16);
    assert_eq!(second_set.size(), 16);

    assert!(!first_set.contains(b'\x00'));
    assert!(!second_set.contains(b'\x00'));

    for c in first_half {
        assert!(first_set.contains(*c));
        assert!(!second_set.contains(*c), "{}", *c as char);
    }
    for c in second_half {
        assert!(!first_set.contains(*c));
        assert!(second_set.contains(*c));
    }

    first_half.iter().for_each(|c| first_set.remove(*c));
    second_half.iter().for_each(|c| second_set.remove(*c));

    assert_eq!(first_set.size(), 0);
}

#[test]
fn iter() {
    let first_half: &[u8] = &SpecialSet::CHARS[0..16];
    let first_set: SpecialSet =
        SpecialSet::from(unsafe { std::str::from_utf8_unchecked(first_half) });
    let result: Vec<u8> = first_set.iter().collect();
    assert_eq!(first_half.to_vec(), result.clone());

    let second_half: &[u8] = &SpecialSet::CHARS[16..32];
    let second_set: SpecialSet =
        SpecialSet::from(unsafe { std::str::from_utf8_unchecked(second_half) });
    let result: Vec<u8> = second_set.iter().collect();
    assert_eq!(second_half, result);
}
