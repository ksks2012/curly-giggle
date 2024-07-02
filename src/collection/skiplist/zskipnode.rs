use std::iter;
use std::ptr::NonNull;

pub type Link<T> = Option<NonNull<ZSkipNode<T>>>;

#[derive(Clone, Debug)]
pub struct ZSkipNode<T> {
    pub val: Option<T>,
    pub level: usize,
    pub next: Vec<Link<T>>,
}

impl<T> ZSkipNode<T> {
    pub fn head(level_bound: usize) -> Self {
        ZSkipNode {
            val: None,
            level: level_bound - 1,
            next: iter::repeat(None).take(level_bound).collect(),
        }
    }

    pub fn new(item: T, level: usize) -> Self {
        ZSkipNode {
            val: Some(item),
            level,
            next: iter::repeat(None).take(level + 1).collect(),
        }
    }
}
