use bits::{Bits, Word};
use std::borrow::Cow;
use std::iter::successors;

#[test]
fn bits_is_implemented() {
    fn _bits_is_implemented<T: ?Sized + Bits>() {}

    _bits_is_implemented::<&u8>();
    _bits_is_implemented::<[u8; 1]>();
    _bits_is_implemented::<&[u8; 1]>();
    _bits_is_implemented::<&[u8]>();
    _bits_is_implemented::<Vec<[u8; 1]>>();
    _bits_is_implemented::<&Vec<[u8; 2]>>();
    _bits_is_implemented::<Box<[u8; 3]>>();
    _bits_is_implemented::<&Box<[u8; 4]>>();
    _bits_is_implemented::<Cow<[u8; 1000]>>();
    _bits_is_implemented::<Cow<Box<[u8; 2000]>>>();
}

fn ones<T: Bits + Word>(word: T) -> impl Iterator<Item = usize> {
    successors(Some(word), |&n| {
        let m = n & !n.lsb();
        Bits::any(&m).then(|| m)
    })
    .map(Word::count_t0)
}

#[test]
fn next_set_bit() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);

    assert_eq!(ones.next(), Some(0));
    assert_eq!(ones.next(), Some(2));
    assert_eq!(ones.next(), Some(4));
    assert_eq!(ones.next(), Some(6));
    assert_eq!(ones.next(), None);
}

#[test]
fn ones_select1() {
    let n: u32 = 0b_0101_0101;
    let mut ones = ones(n);
    for c in 0..64 {
        assert_eq!(ones.next(), Bits::select_1(&n, c));
    }
}