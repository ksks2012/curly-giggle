pub mod zskiplist;
pub mod zskipnode;
pub mod level_generator;


// layer level
const ZSKIPLIST_MAXLEVEL: usize = 32;
const ZSKIPLIST_P: f64 = 0.25;