//! `bits`

pub mod all;
pub mod any;
pub mod bits;
pub mod count;
pub mod get;
pub mod ops;
pub mod put;
pub mod rank;
pub mod select;

mod block;
mod word;
pub use self::{block::Block, word::Word};

#[inline]
fn address<T: Block>(i: usize) -> (usize, usize) {
    use core::ops::{Div, Rem};
    fn divrem<T, U>(t: T, u: U) -> (<T as Div<U>>::Output, <T as Rem<U>>::Output)
    where
        T: Copy + Div<U> + Rem<U>,
        U: Copy,
    {
        (t / u, t % u)
    }

    divrem(i, T::BITS)
}

/// A utility to clamp the given range into a valid one.
/// Panics if debug is enabled and `min <= i && i <= j && j <= max`.
fn to_range<R: core::ops::RangeBounds<usize>>(r: &R, min: usize, max: usize) -> (usize, usize) {
    use core::ops::Bound::*;

    let (i, j) = (
        match r.start_bound() {
            Included(&s) => s,
            Excluded(&s) => s + 1,
            Unbounded => min,
        },
        match r.end_bound() {
            Included(&e) => e + 1,
            Excluded(&e) => e,
            Unbounded => max,
        },
    );

    debug_assert!(min <= i && i <= j && j <= max);
    (i, j)
}

/// Calculates the minimum number of blocks to store `n` bits.
const fn blocks(n: usize, b: usize) -> usize {
    n / b + (n % b > 0) as usize
}

/// Returns an empty `Vec<T>` with the at least specified capacity in bits.
///
/// ```
/// # use bits::ops::Bits;
/// let v = bits::with_capacity::<u8>(80);
/// // v has no bits, but an enough capacity to store 80 bits.
/// assert_eq!(v.bits(), 0);
/// assert_eq!(v.capacity(), 10);
/// ```
pub fn with_capacity<T: Block>(capacity: usize) -> Vec<T> {
    Vec::with_capacity(blocks(capacity, T::BITS))
}

// pub fn null<T: Block>(n: usize) -> Vec<T> {
//     use core::iter::from_fn;
//     let size = blocks(n, T::BITS);
//     from_fn(|| Some(T::empty())).take(size).collect()
// }
