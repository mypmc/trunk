use crate::ops::Count;
use crate::Block;

pub trait All: Count {}

impl All for bool {
    #[inline]
    fn all(&self) -> bool {
        *self
    }
}

impl<T: Block> All for [T] {
    #[inline]
    fn all(&self) -> bool {
        self.iter().all(All::all)
    }
}

macro_rules! impl_all {
    ($X:ty $(, $method:ident )?) => {
        #[inline]
        fn all(&self) -> bool {
            <$X as All>::all(self$(.$method())?)
        }
    }
}

impl<'a, T: ?Sized + All> All for &'a T {
    impl_all!(T);
}

impl<T, const N: usize> All for [T; N]
where
    [T]: All,
{
    impl_all!([T], as_ref);
}

mod alloc {
    use super::*;
    use std::borrow::Cow;

    impl<T> All for Vec<T>
    where
        [T]: All,
    {
        impl_all!([T]);
    }

    impl<T: ?Sized + All> All for Box<T> {
        impl_all!(T);
    }

    impl<'a, T> All for Cow<'a, T>
    where
        T: ?Sized + ToOwned + All,
    {
        impl_all!(T, as_ref);
    }
}
