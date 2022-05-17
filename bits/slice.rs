#![allow(clippy::many_single_char_names)]
use crate::prelude::*;
use core::ops::Range;

fn for_each_blocks<T, F>(s: usize, e: usize, mut f: F)
where
    T: Block,
    F: FnMut(usize, Range<usize>),
{
    assert!(s <= e);
    if s == e {
        return;
    }

    let (q0, r0) = address::<T>(s);
    let (q1, r1) = address::<T>(e);

    if q0 == q1 {
        f(q0, r0..r1);
    } else {
        f(q0, r0..T::BITS);
        (q0 + 1..q1).for_each(|k| f(k, 0..T::BITS));
        f(q1, 0..r1)
    }
}

impl<T> Bits for [T]
where
    T: Block,
{
    #[inline]
    fn len(this: &Self) -> usize {
        T::BITS * <[T]>::len(this)
    }

    #[inline]
    fn get(this: &Self, i: usize) -> Option<bool> {
        let (i, o) = address::<T>(i);
        this.get(i)
            .map(|block| Bits::get(block, o).expect("index out of bounds"))
    }

    #[inline]
    fn count_1(&self) -> usize {
        self.iter().map(Bits::count_1).sum()
    }

    #[inline]
    fn count_0(&self) -> usize {
        self.iter().map(Bits::count_0).sum()
    }

    #[inline]
    fn all(&self) -> bool {
        self.iter().all(Bits::all)
    }

    #[inline]
    fn any(&self) -> bool {
        self.iter().any(Bits::any)
    }

    #[inline]
    fn rank_1<R: RangeBounds<usize>>(&self, r: R) -> usize {
        let (s, e) = to_range(&r, 0, Bits::len(self));
        let (i, p) = address::<T>(s);
        let (j, q) = address::<T>(e);
        if i == j {
            self[i].rank_1(p..q)
        } else {
            self[i].rank_1(p..)
                + self[i + 1..j].count_1()
                + self.get(j).map_or(0, |b| b.rank_1(..q))
        }
    }

    #[inline]
    fn select_1(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_1();
            if n < count {
                return Some(i * T::BITS + b.select_1(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    fn select_0(&self, mut n: usize) -> Option<usize> {
        for (i, b) in self.iter().enumerate() {
            let count = b.count_0();
            if n < count {
                return Some(i * T::BITS + b.select_0(n).expect("BUG"));
            }
            n -= count;
        }
        None
    }

    #[inline]
    #[doc(hidden)]
    fn word<N: Word>(&self, i: usize, n: usize) -> N {
        let mut cur = 0;
        let mut out = N::NULL;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() && cur < <N as Block>::BITS {
                out |= self[k].word::<N>(r.start, r.len()) << cur;
                cur += r.len();
            }
        });
        out
    }
}

impl<T> BitsMut for [T]
where
    T: Block,
{
    #[inline]
    fn put_1(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put_1(o);
    }

    #[inline]
    fn put_0(&mut self, i: usize) {
        assert!(i < Bits::len(self));
        let (i, o) = address::<T>(i);
        self[i].put_0(o);
    }

    #[inline]
    #[doc(hidden)]
    fn put_n<N: Word>(&mut self, i: usize, n: usize, mask: N) {
        let mut cur = 0;
        for_each_blocks::<T, _>(i, i + n, |k, r| {
            if k < self.len() {
                self[k].put_n::<N>(r.start, r.len(), mask.word(cur, r.len()));
                cur += r.len();
            }
        });
    }
}