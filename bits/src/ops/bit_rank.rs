use super::BitCount;
use crate as bits;
use core::ops::RangeBounds;

pub trait BitRank: BitCount {
    /// Counts occurrences of `1` in the given range.
    #[inline]
    fn rank_1<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        (j - i) - self.rank_0(index)
    }

    /// Counts occurrences of `0` in the given range.
    #[inline]
    fn rank_0<Index: RangeBounds<usize>>(&self, index: Index) -> usize {
        let (i, j) = bits::to_range(&index, 0, bits::len(self));
        (j - i) - self.rank_1(index)
    }
}