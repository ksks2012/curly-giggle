use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

use crate::collection::skiplist::{ZSKIPLIST_MAXLEVEL, ZSKIPLIST_P};

use super::zskipnode::{Link, ZSkipNode};
use super::level_generator::{DefaultLevelGenerator, LevelGenerator};

type Comparator<T> = Box<dyn Fn(&T, &T) -> Ordering>;

#[allow(dead_code, unused_variables)]
pub struct ZSkipList<T> {
    header: NonNull<ZSkipNode<T>>,
    tail: NonNull<ZSkipNode<T>>,
    cur_level: usize,
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
        let new_node = NonNull::new(Box::into_raw(Box::new(ZSkipNode::head(g.level_bound())))).unwrap();
        ZSkipList {
            header: new_node,
            tail: new_node,
            cur_level: 0,
            len: 0,
            cmp: Box::new(|x, y| x.cmp(y)),
            _boo: PhantomData,
            level_generator: Box::new(g),
        }
    }
}
   
impl<T: fmt::Debug + std::clone::Clone> fmt::Debug for ZSkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            
            println!("=====================");
            for i in 0..self.cur_level {
                println!("Level {}:", i);
                let mut cur = self.header.as_ref();
                print!("None,({}),None -> ", cur.get_span(i));
                while let Some(next_node) = cur.level[i].forward {
                    let next_node = next_node.as_ref();
                    // val, span, score
                    write!(f, "{:?},({:?}),({:?}) -> ", <ZSkipNode<T> as Clone>::clone(&next_node).into_val(), <ZSkipNode<T> as Clone>::clone(&next_node).get_span(i), <ZSkipNode<T> as Clone>::clone(&next_node).score)?;
                    cur = next_node;
                }
                println!("\n");
            }
            println!("=====================");
        }
        Ok(())
    }
}

impl<T> ZSkipList<T> {
    pub fn iter(&self) -> Iter<T> {
        let node = unsafe { self.header.as_ref().level[0].forward };
        
        Iter {
            head: node,
            len: self.len,
            _boo: PhantomData,
        }
    }
        
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let node = unsafe { self.header.as_ref().level[0].forward };
        
        IterMut {
            head: node,
            len: self.len,
            _boo: PhantomData,
        }
    }      

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let first_node = self.header.as_ref().level[0].forward.as_ref();
            match first_node {
                Some(node) => self.zsl_delete(node.as_ref().val.as_ref().unwrap()),
                None => None,
            }
        }
    }

    // TODO: Optimization
    pub fn contains(&self, v: &T) -> bool {
        unsafe {
            let mut cur: &ZSkipNode<T> = self.header.as_ref();
            for i in (0..self.cur_level).rev() {
                while let Some(next_node) = cur.level[i].forward {
                    let next_node = next_node.as_ref();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), v) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                if let Some(next_node) = cur.level[i].forward {
                    let next_node = next_node.as_ref();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), v) == Ordering::Equal {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_len(&self) -> usize {
        return self.len;
    }
}

impl<T: std::clone::Clone> ZSkipList<T> {
    pub fn zsl_get_element_by_rank(&self, rank: usize) -> Option<T> {
        // Get the element at the given rank in the skip list
        // Return None if the rank is out of range
        let mut traversed : usize = 0;
        unsafe {
            let mut cur: &ZSkipNode<T> = self.header.as_ref();
            for i in (0..self.cur_level).rev() {
                while let Some(forward) = cur.level[i].forward {
                    if cur.level[i].span + traversed <= rank {
                        
                        traversed += cur.level[i].span;
                        cur = forward.as_ref();
                    } else {
                        break;
                    }
                }
                if traversed == rank {
                    return cur.val.as_ref().cloned();
                }
            }
        }
        return None;
    }

    pub fn zsl_first_in_range(&self, min: f64, max: f64) -> Option<T> {
        // Get the first element in the skip list within the given score range
        // Return None if there are no elements in the range
        unsafe {
            let mut cur = self.header.as_ref();

            // Start from the highest level and move downwards
            for i in (0..1).rev() {
                // Traverse forward nodes at the current level
                while let Some(forward) = cur.level[i].forward {
                    let forward_node = forward.as_ref();
                    if forward_node.score > max {
                        // If the next node's score is greater than max, break out of the loop
                        break;
                    }

                    if forward_node.score >= min {
                        // If the next node's score is within the range, return value
                        return forward_node.val.as_ref().cloned();
                    }

                    // Move to the next node
                    cur = forward_node;
                }
            }
        }
        None
    }

    pub fn zsl_last_in_range(&self, min: f64, max: f64) -> Option<T> {
        // Get the last element in the skip list within the given score range
        // Return None if there are no elements in the range
        unsafe {
            let mut cur = self.header.as_ref();
    
            // Start from the highest level and move downwards
            for i in (0..1).rev() {
                // Traverse forward nodes at the current level
                while let Some(forward) = cur.level[i].forward {
                    let forward_node = forward.as_ref();
                    
                    if forward_node.score > max {
                        // If the next node's score is greater than max, break out of the loop
                        break;
                    }
                    cur = forward_node;
                }
            }
    
            // Check if the current node is within the range
            if cur.score >= min && cur.score <= max {
                return cur.val.as_ref().cloned();
            }
        }
        None
    }
}

#[allow(dead_code, unused_variables)]
impl<T> ZSkipList<T> {

    pub fn zsl_free(&mut self) {
        // TODO: Implementation of zsl_free here
        // Free any allocated memory and clean up resources
    }

    /// Insert the element with the given score into the skip list.
    ///
    /// If the element already exists in the skip list, it will not be inserted again.
    ///
    /// # Arguments
    /// 
    /// * `score` - The score associated with the element. The score is used to determine the position of the element in the skip list.
    /// * `element` - The element to be inserted into the skip list.
    ///
    /// # Examples
    ///
    /// ```
    /// use curly_giggle::collection::skiplist::zskiplist::ZSkipList;
    ///
    /// let mut skip_list = ZSkipList::zsl_create();
    /// skip_list.zsl_insert(1.0, 42);
    /// ```
    pub fn zsl_insert(&mut self, score: f64, element: T) -> NonNull<ZSkipNode<T>> {
        let mut update = vec![self.header; ZSKIPLIST_MAXLEVEL];
        let mut rank: Vec<usize> = vec![0; ZSKIPLIST_MAXLEVEL];

        let mut x = unsafe { self.header.as_ref() };
        for i in (0..self.cur_level).rev() {
            rank[i] = if i == self.cur_level - 1 { 0 } else { rank[i + 1] };
            while let Some(forward) = x.level[i].forward {
                let forward_node: &ZSkipNode<T> = unsafe {forward.as_ref()};
                // Check score and value
                if forward_node.score < score 
                    || (forward_node.score == score 
                    && (self.cmp)(forward_node.val.as_ref().unwrap(), &element) == Ordering::Less) 
                {
                    rank[i] += x.level[i].span;
                    x = forward_node;
                } else {
                    break;
                }
            }
            update[i] = NonNull::from(x);
        }

        let level = self.level_generator.random();

        // initialize the level of the new node
        if level > self.cur_level {
            for i in self.cur_level..level {
                rank[i] = 0;
                update[i] = self.header;
                unsafe { self.header.as_mut().level[i].span = self.len };
            }
            self.cur_level = level;
        }

        let new_node = Box::new(ZSkipNode::new(element, level, score));
        let mut new_node_ptr = NonNull::new(Box::into_raw(new_node)).unwrap();

        // Update the forward pointers
        unsafe {
            for i in 0..level {
            
                let update_node =  update[i].as_mut();

                new_node_ptr.as_mut().level[i].forward = update_node.level[i].forward;
                update_node.level[i].forward = Some(new_node_ptr);

                new_node_ptr.as_mut().level[i].span = update_node.level[i].span - (rank[0] - rank[i]);
                update_node.level[i].span = (rank[0] - rank[i]) + 1;
            }
        }

        for i in level..self.cur_level {
            unsafe { update[i].as_mut().level[i].span += 1 };
        }

        unsafe {
            new_node_ptr.as_mut().backward = if update[0] == self.header { None } else { Some(update[0]) };
            if let Some(mut forward) = new_node_ptr.as_mut().level[0].forward {
                forward.as_mut().backward = Some(new_node_ptr);
            } else {
                self.tail = new_node_ptr;
            }
        }

        self.len += 1;

        new_node_ptr
    }

    /// Delete the element with the given score from the skip list.
    ///
    /// If the element is found and deleted, it will be returned as `Some(element)`.
    /// If the element is not found, `None` will be returned.
    ///
    /// # Arguments
    ///
    /// * `element` - The element to be deleted from the skip list.
    ///
    /// # Examples
    ///
    /// ```
    /// use curly_giggle::collection::skiplist::zskiplist::ZSkipList;
    ///
    /// let mut skip_list = ZSkipList::zsl_create();
    /// skip_list.zsl_insert(1.0, 42);
    /// assert_eq!(skip_list.zsl_delete(&42), Some(42));
    /// assert_eq!(skip_list.zsl_delete(&42), None);
    /// ```
    pub fn zsl_delete(&mut self, element: &T) -> Option<T>{
        if !self.contains(&element) {
            return None;
        }

        let mut cur = unsafe { self.header.as_mut() };
        // TODO: level
        let max_level = self.cur_level;
        let mut update: Vec<Option<*mut ZSkipNode<T>>> = vec![None; max_level + 1];
        let res_val;
        unsafe {
            for i in (0..=max_level).rev() {
                while let Some(mut next_node) = cur.level[i].forward {
                    let next_node = next_node.as_mut();
                    if (self.cmp)(next_node.val.as_ref().unwrap(), &element) == Ordering::Less {
                        cur = next_node;
                    } else {
                        break;
                    }
                }
                update[i] = Some(cur as *mut ZSkipNode<T>);
            }

            let mut ret_val_ref = None;
            if cur.level[0].forward.is_some()
                && (self.cmp)(cur.level[0].forward.unwrap().as_mut().val.as_ref().unwrap(), &element)
                    == Ordering::Equal
            {
                ret_val_ref = cur.level[0].forward;
                for i in (0..=max_level).rev() {
                    if update[i].is_some()
                        && (*update[i].unwrap()).level[i].forward.is_some()
                        && (self.cmp)(
                            (*update[i].unwrap()).level[i].forward
                                .unwrap()
                                .as_mut()
                                .val
                                .as_ref()
                                .unwrap(),
                            &element,
                        ) == Ordering::Equal
                    {
                        (*update[i].unwrap()).level[i].forward =
                            (*update[i].unwrap()).level[i].forward.unwrap().as_mut().level[i].forward;
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

    pub fn zsl_get_rank(&self, score: f64, element: T) -> Option<i128> {
        // Get the rank of the element with the given score in the skip list
        // Return None if the element is not found
        let mut rank: Option<i128> = Some(0);
        unsafe {
            let mut cur: &ZSkipNode<T> = self.header.as_ref();
            for i in (0..self.cur_level).rev() {
                while let Some(forward) = cur.level[i].forward {
                    let forward_node: &ZSkipNode<T> = forward.as_ref();
                    if forward_node.score < score 
                        || (forward_node.score == score 
                        && ((self.cmp)(forward_node.val.as_ref().unwrap(), &element) == Ordering::Less))
                    {
                        rank = rank.map(|r| r + cur.level[i].span as i128);
                        cur = forward_node;
                    } else {
                        break;
                    }
                }

                if let Some(next_node) = cur.level[i].forward {
                    let next_node = next_node.as_ref();
                    if !cur.is_head() && (self.cmp)(next_node.val.as_ref().unwrap(), &element) == Ordering::Equal {
                        return rank;
                    } else if i == 0 && (self.cmp)(next_node.val.as_ref().unwrap(), &element) == Ordering::Equal {
                        return Some(0);
                    }
                }
            }
        }
        return None;
    }

    pub fn zsl_is_in_range(&self, min: f64, max: f64) -> bool {
        // Check if there are any elements in the skip list within the given score range
        unsafe {
            let mut cur = self.header.as_ref();

            // Start from the highest level and move downwards
            for i in (0..self.cur_level).rev() {
                // Traverse forward nodes at the current level
                while let Some(forward) = cur.level[i].forward {
                    let forward_node = forward.as_ref();

                    if forward_node.score > max {
                        // If the next node's score is greater than max, break out of the loop
                        break;
                    }

                    if forward_node.score >= min {
                        // If the next node's score is within the range, return true
                        return true;
                    }

                    // Move to the next node
                    cur = forward_node;
                }
            }
        }

        false
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
                        self.head = node.level[0].forward;
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
                        self.head = node.level[0].forward;
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