use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

use super::zskipnode::{Link, ZSkipNode};
use super::level_generator::{DefaultLevelGenerator, LevelGenerator};

type Comparator<T> = Box<dyn Fn(&T, &T) -> Ordering>;

#[allow(dead_code, unused_variables)]
pub struct ZSkipList<T> {
    header: NonNull<ZSkipNode<T>>,
    level: usize,
    len: usize,
    cmp: Comparator<T>,
    _boo: PhantomData<T>,
    level_generator: Box<dyn LevelGenerator>, 
}

#[allow(dead_code, unused_variables)]
impl<T: Ord> ZSkipList<T> {
    pub fn zsl_create() -> Self {
        // TODO: Implementation of zsl_create here
        // LevelGenerator
        let g = DefaultLevelGenerator::default();
        ZSkipList {
            header: NonNull::new(Box::into_raw(Box::new(ZSkipNode::head(g.level_bound())))).unwrap(),
            level: 0,
            len: 0,
            cmp: Box::new(|x, y| x.cmp(y)),
            _boo: PhantomData,
            level_generator: Box::new(g),
        }
    }
}

impl<T> ZSkipList<T> {
    pub fn iter(&self) -> Iter<T> {
        let node = unsafe { self.header.as_ref().next[0] };
        
        Iter {
            head: node,
            len: self.len,
            _boo: PhantomData,
        }
    }
        
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let node = unsafe { self.header.as_ref().next[0] };
        
        IterMut {
            head: node,
            len: self.len,
            _boo: PhantomData,
        }
    }      

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let first_node = self.header.as_ref().next[0].as_ref();
            match first_node {
                Some(node) => self.remove(node.as_ref().val.as_ref().unwrap()),
                None => None,
            }
        }
    }

    pub fn contains(&self, v: &T) -> bool {
        unsafe {
            let mut cur = self.header.as_ref();

            for i in (0..=cur.level).rev() {
                while let Some(next_node) = cur.next[i] {
                    let next_node = next_node.as_ref();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), v) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                if cur.next[i].is_some()
                    && (self.cmp)(cur.next[i].unwrap().as_ref().val.as_ref().unwrap(), v)
                        == Ordering::Equal
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn remove(&mut self, val: &T) -> Option<T> {
        if !self.contains(val) {
            return None;
        }

        let mut cur = unsafe { self.header.as_mut() };
        let max_level = cur.level;
        let mut update: Vec<Option<*mut ZSkipNode<T>>> = vec![None; max_level + 1];
        let res_val;
        unsafe {
            for i in (0..=max_level).rev() {
                while let Some(mut next_node) = cur.next[i] {
                    let next_node = next_node.as_mut();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), val) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                update[i] = Some(cur as *mut ZSkipNode<T>);
            }

            let mut ret_val_ref = None;
            if cur.next[0].is_some()
                && (self.cmp)(cur.next[0].unwrap().as_ref().val.as_ref().unwrap(), val)
                    == Ordering::Equal
            {
                ret_val_ref = cur.next[0];
                for i in (0..=max_level).rev() {
                    if update[i].is_some()
                        && (*update[i].unwrap()).next[i].is_some()
                        && (self.cmp)(
                            (*update[i].unwrap()).next[i]
                                .unwrap()
                                .as_mut()
                                .val
                                .as_ref()
                                .unwrap(),
                            val,
                        ) == Ordering::Equal
                    {
                        (*update[i].unwrap()).next[i] =
                            (*update[i].unwrap()).next[i].unwrap().as_mut().next[i];
                    }
                }
            }
            res_val = match ret_val_ref {
                None => None,
                Some(ret) => Box::from_raw(ret.as_ptr()).into_val(),
            }
        }

        self.len -= 1;

        res_val
    }
}

#[allow(dead_code, unused_variables)]
impl<T> ZSkipList<T> {

    pub fn zsl_free(&mut self) {
        // TODO: Implementation of zsl_free here
        // Free any allocated memory and clean up resources
    }

    pub fn zsl_insert(&mut self, score: f64, element: T) {
        // TODO: Implementation of zsl_insert here
        // Insert the element with the given score into the skip list
    }

    pub fn zsl_delete(&mut self, score: f64, element: T) {
        // TODO: Implementation of zsl_delete here
        // Delete the element with the given score from the skip list
    }

    pub fn zsl_get_rank(&self, score: f64, element: T) -> Option<usize> {
        // TODO: Implementation of zsl_get_rank here
        // Get the rank of the element with the given score in the skip list
        // Return None if the element is not found
        None
    }

    pub fn zsl_get_element_by_rank(&self, rank: usize) -> Option<String> {
        // TODO: Implementation of zsl_get_element_by_rank here
        // Get the element at the given rank in the skip list
        // Return None if the rank is out of range
        None
    }

    pub fn zsl_is_in_range(&self, min: f64, max: f64) -> bool {
        // TODO: Implementation of zsl_is_in_range here
        // Check if there are any elements in the skip list within the given score range
        false
    }

    pub fn zsl_first_in_range(&self, min: f64, max: f64) -> Option<String> {
        // TODO: Implementation of zsl_first_in_range here
        // Get the first element in the skip list within the given score range
        // Return None if there are no elements in the range
        None
    }

    pub fn zsl_last_in_range(&self, min: f64, max: f64) -> Option<String> {
        // TODO: Implementation of zsl_last_in_range here
        // Get the last element in the skip list within the given score range
        // Return None if there are no elements in the range
        None
    }

    pub fn zsl_delete_range_by_score(&mut self, min: f64, max: f64) -> usize {
        // TODO: Implementation of zsl_delete_range_by_score here
        // Delete all elements in the skip list within the given score range
        // Return the number of elements deleted
        0
    }

    pub fn zsl_delete_range_by_rank(&mut self, start: usize, end: usize) -> usize {
        // TODO: Implementation of zsl_delete_range_by_rank here
        // Delete all elements in the skip list within the given rank range
        // Return the number of elements deleted
        0
    }
}

impl<T> Drop for ZSkipList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut ZSkipList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}

pub struct Iter<'a, T: 'a> {
    head: Link<T>,
    len: usize,
    _boo: PhantomData<&'a ZSkipList<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match self.head {
                Some(node) => {
                    self.len -= 1;

                    unsafe {
                        let node = &*node.as_ptr();
                        self.head = node.next[0];
                        node.val.as_ref()
                    }
                }
                None => None,
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut ZSkipNode<T>>,
}

/// An iterator over mutable references to the elements of a `ZSkipList`.
///
/// This struct is created by the [`iter_mut`] method on [`ZSkipList`].
///
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    /// Advances the iterator and returns the next mutable reference to the element.
    ///
    /// Returns [`None`] if there are no more elements to iterate over.
    ///
    /// # Safety
    ///
    /// This method uses unsafe code to obtain a mutable reference to the element. It is the caller's
    /// responsibility to ensure that the iterator is not used after the element has been modified or
    /// removed from the `ZSkipList`.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            match self.head {
                Some(node) => {
                    self.len -= 1;

                    unsafe {
                        let node = &mut *node.as_ptr();
                        self.head = node.next[0];
                        node.val.as_mut()
                    }
                }
                None => None,
            }
        }
    }

    /// Returns a hint of the number of elements remaining in the iterator.
    ///
    /// The returned value is a tuple `(lower, upper)`, where `lower` is the exact number of elements
    /// remaining and `upper` is an optional upper bound on the number of elements remaining.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IntoIter<T> {
    list: ZSkipList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> IntoIterator for ZSkipList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}
    }
}