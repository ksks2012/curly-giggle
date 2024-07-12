use rand::rngs::SmallRng;
use rand::{thread_rng, Rng};
use rand::SeedableRng;
use crate::collection::skiplist::ZSKIPLIST_MAXLEVEL;

pub trait LevelGenerator {
    fn level_bound(&self) -> usize;
    fn random(&mut self) -> usize;
}

pub struct DefaultLevelGenerator {
    level_bound: usize,
    p: f64,
    rng: SmallRng,
}

impl Default for DefaultLevelGenerator {
    fn default() -> Self {
        DefaultLevelGenerator::new(16, 1.0 / 2.0).unwrap()
    }
}

/// The default level generator for the skip list.
impl DefaultLevelGenerator {
    /// Creates a new instance of the default level generator.
    ///
    /// # Arguments
    ///
    /// * `level_bound` - The maximum number of levels in the skip list.
    /// * `p` - The probability used to determine the number of levels for a new node.
    ///
    /// # Errors
    ///
    /// Returns an error if `level_bound` is zero or if `p` is not in the range (0, 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use curly_giggle::collection::skiplist::level_generator::DefaultLevelGenerator;
    ///
    /// let level_generator = DefaultLevelGenerator::new(4, 0.5);
    /// assert!(level_generator.is_ok());
    /// ```
    pub fn new(level_bound: usize, p: f64) -> Result<Self, String> {
        if level_bound == 0 {
            return Err("total must be non-zero.".to_string());
        }
        if (p - 0.0).abs() < 1e-3 || (p - 1.0).abs() < 1e-3 {
            return Err("p must be in (0, 1).".to_string());
        }
        Ok(DefaultLevelGenerator {
            level_bound,
            p,
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        })
    }
}

/// Implementation of the `LevelGenerator` trait for the `DefaultLevelGenerator` struct.
impl LevelGenerator for DefaultLevelGenerator {
    /// Returns the level bound for the skiplist.
    fn level_bound(&self) -> usize {
        self.level_bound
    }

    /// Generates a random level for a new skiplist node.
    fn random(&mut self) -> usize {
        let mut level = 0;
        let mut x = self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && level + 1 < self.level_bound {
            level += 1;
            x *= self.p
        }
        level
    }
}

pub fn generate_random_level() -> usize {
    let mut level = 1;
    // The random number generator used for generating levels in the skip list.
    let mut rng = thread_rng();

    for _i in 1..ZSKIPLIST_MAXLEVEL {
        if rng.gen::<u32>() % 2 == 1 {
            level += 1;
        }
    }
    level
}
