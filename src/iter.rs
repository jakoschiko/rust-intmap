#![allow(clippy::type_complexity)]

use std::iter::FlatMap as IterFlatMap;
use std::iter::Flatten as IterFlatten;
use std::slice::Iter as SliceIter;
use std::slice::IterMut as SliceIterMut;
use std::vec::Drain as VecDrain;
use std::vec::IntoIter as VecIntoIter;

use crate::IntKey;
use crate::IntMap;

// ***************** Iter *********************

pub struct Iter<'a, K: IntKey, V: 'a> {
    inner: IterFlatten<SliceIter<'a, Vec<(K::Int, V)>>>,
}

impl<'a, K: IntKey, V> Iter<'a, K, V> {
    pub(crate) fn new(vec: &'a [Vec<(K::Int, V)>]) -> Self {
        Iter {
            inner: vec.iter().flatten(),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (int, value) = self.inner.next()?;
        let key = K::from_int(*int);
        Some((key, value))
    }
}

// ***************** Iter Mut *********************

pub struct IterMut<'a, K: IntKey, V: 'a> {
    inner: IterFlatten<SliceIterMut<'a, Vec<(K::Int, V)>>>,
}

impl<'a, K: IntKey, V> IterMut<'a, K, V> {
    pub(crate) fn new(vec: &'a mut [Vec<(K::Int, V)>]) -> IterMut<'a, K, V> {
        IterMut {
            inner: vec.iter_mut().flatten(),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (int, value) = self.inner.next()?;
        let key = K::from_int(*int);
        Some((key, value))
    }
}

// ***************** Keys Iter *********************

pub struct Keys<'a, K: IntKey, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for Keys<'a, K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|kv| kv.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Iter *********************

pub struct Values<'a, K: IntKey, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|kv| kv.1)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Mut *********************

pub struct ValuesMut<'a, K: IntKey, V: 'a> {
    pub(crate) inner: IterMut<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|kv| kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Into Iter *********************

impl<K: IntKey, V> IntoIterator for IntMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.cache)
    }
}

pub struct IntoIter<K: IntKey, V> {
    inner: IterFlatten<VecIntoIter<Vec<(K::Int, V)>>>,
}

impl<K: IntKey, V> IntoIter<K, V> {
    pub(crate) fn new(vec: Vec<Vec<(K::Int, V)>>) -> Self {
        IntoIter {
            inner: vec.into_iter().flatten(),
        }
    }
}

impl<K: IntKey, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (int, value) = self.inner.next()?;
        let key = K::from_int(int);
        Some((key, value))
    }
}

// ***************** Drain Iter *********************

#[allow(clippy::type_complexity)]
pub struct Drain<'a, K: IntKey, V: 'a> {
    count: &'a mut usize,
    inner: IterFlatMap<
        SliceIterMut<'a, Vec<(K::Int, V)>>,
        VecDrain<'a, (K::Int, V)>,
        fn(&mut Vec<(K::Int, V)>) -> VecDrain<(K::Int, V)>,
    >,
}

impl<'a, K: IntKey, V> Drain<'a, K, V> {
    pub(crate) fn new(vec: &'a mut [Vec<(K::Int, V)>], count: &'a mut usize) -> Drain<'a, K, V> {
        Drain {
            count,
            inner: vec.iter_mut().flat_map(|v| v.drain(..)),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (int, value) = self.inner.next()?;
        *self.count -= 1;
        let key = K::from_int(int);
        Some((key, value))
    }
}

// ***************** Extend *********************

impl<K: IntKey, V> Extend<(K, V)> for IntMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem.0, elem.1);
        }
    }
}

// ***************** FromIterator *********************

impl<K: IntKey, V> std::iter::FromIterator<(K, V)> for IntMap<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        let mut map = IntMap::with_capacity(lower_bound);
        for elem in iterator {
            map.insert(elem.0, elem.1);
        }

        map
    }
}
