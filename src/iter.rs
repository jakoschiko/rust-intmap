use std::iter::FlatMap as IterFlatMap;
use std::iter::Flatten as IterFlatten;
use std::slice::Iter as SliceIter;
use std::slice::IterMut as SliceIterMut;
use std::vec::IntoIter as VecIntoIter;

use smallvec::Drain as SmallVecDrain;
use smallvec::SmallVec;

use crate::IntMap;

// ***************** Iter *********************

pub struct Iter<'a, K: 'a, V: 'a> {
    inner: IterFlatten<SliceIter<'a, SmallVec<[(K, V); 1]>>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(crate) fn new(vec: &'a [SmallVec<[(K, V); 1]>]) -> Self {
        Iter {
            inner: vec.iter().flatten(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.inner.next().map(|r| (&r.0, &r.1))
    }
}

// ***************** Iter Mut *********************

pub struct IterMut<'a, K: 'a, V: 'a> {
    inner: IterFlatten<SliceIterMut<'a, SmallVec<[(K, V); 1]>>>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    pub(crate) fn new(vec: &'a mut [SmallVec<[(K, V); 1]>]) -> IterMut<'a, K, V> {
        IterMut {
            inner: vec.iter_mut().flatten(),
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.inner.next().map(|r| (&r.0, &mut r.1))
    }
}

// ***************** Keys Iter *********************

pub struct Keys<'a, K: 'a, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|kv| kv.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Iter *********************

pub struct Values<'a, K: 'a, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|kv| kv.1)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Mut *********************

pub struct ValuesMut<'a, K: 'a, V: 'a> {
    pub(crate) inner: IterMut<'a, K, V>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|kv| kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Into Iter *********************

impl<V> IntoIterator for IntMap<V> {
    type Item = (u64, V);
    type IntoIter = IntoIter<u64, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.cache)
    }
}

pub struct IntoIter<K, V> {
    inner: IterFlatten<VecIntoIter<SmallVec<[(K, V); 1]>>>,
}

impl<K, V> IntoIter<K, V> {
    pub(crate) fn new(vec: Vec<SmallVec<[(K, V); 1]>>) -> Self {
        IntoIter {
            inner: vec.into_iter().flatten(),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next()
    }
}

// ***************** Drain Iter *********************

#[allow(clippy::type_complexity)]
pub struct Drain<'a, K: 'a, V: 'a> {
    count: &'a mut usize,
    inner: IterFlatMap<
        SliceIterMut<'a, SmallVec<[(K, V); 1]>>,
        SmallVecDrain<'a, [(K, V); 1]>,
        fn(&mut SmallVec<[(K, V); 1]>) -> SmallVecDrain<[(K, V); 1]>,
    >,
}

impl<'a, K, V> Drain<'a, K, V> {
    pub(crate) fn new(
        vec: &'a mut [SmallVec<[(K, V); 1]>],
        count: &'a mut usize,
    ) -> Drain<'a, K, V> {
        Drain {
            count,
            inner: vec.iter_mut().flat_map(|v| v.drain(..)),
        }
    }
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        let next = self.inner.next();
        if next.is_some() {
            *self.count -= 1;
        }
        next
    }
}

// ***************** Extend *********************

impl<V> Extend<(u64, V)> for IntMap<V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (u64, V)>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem.0, elem.1);
        }
    }
}

// ***************** FromIterator *********************

impl<V> std::iter::FromIterator<(u64, V)> for IntMap<V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (u64, V)>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        let mut map = IntMap::with_capacity(lower_bound);
        for elem in iterator {
            map.insert(elem.0, elem.1);
        }

        map
    }
}
