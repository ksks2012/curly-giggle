use curly_giggle::sds::SDS;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sds_new() {
        let sds = SDS::new();
        assert_eq!(sds.sdslen(), 0);
        assert_eq!(sds.sdsavail(), 0);
        assert_eq!(sds.sdsbuf(), Vec::<u8>::new());
    }
    
    #[test]
    fn test_sds_to_string() {
        let sds = SDS::sdsnew("Hello, World!");
        assert_eq!(sds.to_string(), "Hello, World!");
    }
    
    #[test]
    fn test_sds_len() {
        let sds = SDS::sdsnew("Hello");
        assert_eq!(sds.sdslen(), 5);
        assert_eq!(sds.sdsavail(), 5);
        assert_eq!(sds.sdsbuf().len(), 10);
        assert_eq!(sds.to_string(), "Hello\0\0\0\0\0");
    }
    
    #[test]
    fn test_sds_empty() {
        let sds = SDS::new();
        assert!(sds.sdsempty());
        
        let sds = SDS::sdsnew("Hello");
        assert!(!sds.sdsempty());
    }
    
    #[test]
    fn test_sds_free() {
        let mut sds = SDS::sdsnew("Hello");
        sds.sdsfree();
        assert_eq!(sds.sdslen(), 0);
        assert_eq!(sds.sdsavail(), 0);
        assert_eq!(sds.sdsbuf(), Vec::<u8>::new());
    }
    
    #[test]
    fn test_sds_dup() {
        let sds = SDS::sdsnew("Hello");
        let sds_dup = sds.sdsdup();
        assert_eq!(sds_dup.sdslen(), sds.sdslen());
        assert_eq!(sds_dup.sdsavail(), sds.sdsavail());
        assert_eq!(sds_dup.sdsbuf(), sds.sdsbuf());
    }
    
    #[test]
    fn test_sds_clear() {
        let mut sds = SDS::sdsnew("Hello");
        sds.sdsclear();
        assert_eq!(sds.sdslen(), 0);
        assert_eq!(sds.sdsavail(), 5);
        assert_eq!(sds.sdsbuf(), Vec::<u8>::new());
    }
    
    #[test]
    fn test_sds_cat() {
        let mut sds = SDS::sdsnew("Hello");
        sds.sdscat(", World!");
        assert_eq!(sds.sdslen(), 13);
        assert_eq!(sds.sdsavail(), 13);
        assert_eq!(sds.to_string(), "Hello, World!\0\0\0\0\0\0\0\0\0\0\0\0\0");
    }
    
    #[test]
    fn test_sds_cat_sds() {
        let mut sds1 = SDS::sdsnew("Hello");
        let sds2 = SDS::sdsnew(", World!");
        sds1.sdscatsds(&sds2);
        assert_eq!(sds1.to_string(), "Hello, World!");
    }
    
    #[test]
    fn test_sds_cpy() {
        let mut sds = SDS::sdsnew("Hello");
        sds.sdscpy("World!");
        assert_eq!(sds.to_string(), "World!");
    }
    
    #[test]
    fn test_sds_grow_zero() {
        let mut sds = SDS::sdsnew("Hello");
        sds.sdsgrowzero(5);
        assert_eq!(sds.to_string(), "Hello     ");
    }
    
    #[test]
    fn test_sds_range() {
        let mut sds = SDS::sdsnew("Hello, World!");
        sds.sdsrange(0, 4);
        assert_eq!(sds.to_string(), "Hello\0 World!");
        assert_eq!(sds.sdsavail(), 8);
        assert_eq!(sds.sdslen(), 5);
        
        let mut sds = SDS::sdsnew("Hello, World!");
        sds.sdsrange(7, 11);
        assert_eq!(sds.to_string(), "World\0 World!");
        assert_eq!(sds.sdsavail(), 8);
        assert_eq!(sds.sdslen(), 5);
    }
    
    #[test]
    fn test_sds_trim() {
        let mut sds = SDS::sdsnew("   Hello, World!   ");
        sds.sdstrim(" ");
        assert_eq!(sds.to_string(), "Hello, World!");
    }
    
    #[test]
    fn test_sds_cmp() {
        let sds1 = SDS::sdsnew("Hello");
        let sds2 = SDS::sdsnew("World");
        assert_eq!(sds1.sdscmp(&sds2), Ordering::Less);
        
        let sds1 = SDS::sdsnew("Hello");
        let sds2 = SDS::sdsnew("Hello");
        assert_eq!(sds1.sdscmp(&sds2), Ordering::Equal);
        
        let sds1 = SDS::sdsnew("World");
        let sds2 = SDS::sdsnew("Hello");
        assert_eq!(sds1.sdscmp(&sds2), Ordering::Greater);
    }
}
