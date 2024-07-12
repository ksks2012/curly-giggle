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
    pub forward: Option<Link<T>>,
    pub span: usize,
}

#[derive(Clone, Debug)]
pub struct ZSkipNode<T> {
    pub val: Option<T>,
    pub score: f64,
    pub backward: Link<T>,
    pub level: Vec<ZSkipLevel<T>>,
    pub next: Vec<Link<T>>,
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
                ZSKIPLIST_MAXLEVEL
            ],
            next: iter::repeat(None).take(level_bound).collect(),
        }
    }

    pub fn new(item: T, level: usize) -> Self {
        ZSkipNode {
            val: Some(item),
            score: 0.0,
            backward: None,
            level: vec![
                ZSkipLevel {
                    forward: None,
                    span: 0,
                };
                ZSKIPLIST_MAXLEVEL
            ],
            next: iter::repeat(None).take(level + 1).collect(),
        }
    }

    pub fn into_val(self) -> Option<T> {
        self.val
    }

    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}
