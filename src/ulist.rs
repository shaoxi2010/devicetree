use alloc::collections::linked_list::{self, LinkedList};
use core::iter::{IntoIterator, Iterator};

pub struct UList<K, V> {
    inter: LinkedList<(K, V)>,
}

impl<K, V> UList<K, V> where K: PartialEq {
    pub fn new() -> Self {
        Self { inter: LinkedList::new() }
    }
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(v) = self.get_mut(&key) {
            *v = value;
        } else {
            self.inter.push_back((key, value))
        }

    }
    pub fn get(&self, k: &K) -> Option<&V> {
        for (tk, tv) in &self.inter {
            if tk == k {
                return Some(&tv)
            }
        }
        None
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        for (tk, tv) in self {
            if tk == k {
                return Some(tv)
            }
        }
        None
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        // FIXME： not impl
        None
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { inter:self.inter.iter() }
    }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    inter: linked_list::IterMut<'a, (K, V)>
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a mut K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inter.next()
            .map(|t| { (&mut t.0, &mut t.1) })
    }
}

impl<'a, K, V> IntoIterator for &'a mut UList<K, V> {
    type Item = (&'a mut K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { inter:self.inter.iter_mut() }
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    inter: linked_list::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inter.next()
            .map(|t| { (&t.0, &t.1) })
    }
}

impl<'a, K, V> IntoIterator for &'a UList<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { inter:self.inter.iter() }
    }
}



pub struct IntoIter<K, V> {
    inter: linked_list::IntoIter<(K, V)>
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inter.next()
    }
}

impl<K, V> IntoIterator for UList<K, V>{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inter: self.inter.into_iter()
        }
    }
}