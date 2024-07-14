use std::iter;
use std::ptr::NonNull;

use crate::collection::skiplist::ZSKIPLIST_MAXLEVEL;

pub type Link<T> = Option<NonNull<ZSkipNode<T>>>;


impl<T> Clone for ZSkipLevel<T> {
    fn clone(&self) -> Self {
        ZSkipLevel {
            forward: self.forward.clone(),
            span: self.span,
        }
    }
}
#[derive(Debug)]
pub struct ZSkipLevel<T> {
    pub forward: Link<T>,
    pub span: usize,
}

#[derive(Clone, Debug)]
pub struct ZSkipNode<T> {
    pub val: Option<T>,
    pub score: f64,
    pub backward: Link<T>,
    pub level: Vec<ZSkipLevel<T>>,
    // NOTE: next been replaced with level
}

impl<T> ZSkipNode<T> {
    pub fn head(level_bound: usize) -> Self {
        ZSkipNode {
            val: None,
            score: 0.0,
            backward: None,
            level: vec![
                ZSkipLevel {
                    forward: None,
                    span: 0,
                };
                level_bound
            ],
        }
    }

    // Creates a new `ZSkipNode` with the specified item and level.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to be stored in the node.
    /// * `level` - The level of the node.
    ///
    /// # Returns
    ///
    /// A new `ZSkipNode` with the specified item and level.
    ///
    /// # Example
    ///
    /// ```
    /// use curly_giggle::collection::skiplist::zskipnode::ZSkipNode;
    ///
    /// let node: ZSkipNode<i32> = ZSkipNode::new(42, 3, 1.0);
    /// assert_eq!(node.level.len(), 3);
    /// ```
    pub fn new(item: T, level: usize, score: f64) -> Self {
        ZSkipNode {
            val: Some(item),
            score: score,
            backward: None,
            level: vec![
                ZSkipLevel {
                    forward: None,
                    span: 0,
                };
                level
            ],
        }
    }

    pub fn into_val(self) -> Option<T> {
        self.val
    }

    pub fn into_item(self) -> T {
        self.val.unwrap()
    }

    pub fn is_head(&self) -> bool {
        self.val.is_none()
    }

    pub fn is_tail(&self) -> bool {
        self.level[0].forward.is_none()
    }

    pub fn level(&self) -> usize {
        self.level.len()
    }

    pub fn get_span(&self, level: usize) -> usize {
        self.level[level].span
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}
