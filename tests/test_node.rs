use curly_giggle::collection::skiplist::zskipnode::ZSkipNode;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head() {
        let level_bound = 5;
        let node = ZSkipNode::<i32>::head(level_bound);

        assert_eq!(node.val, None);
        assert_eq!(node.level, level_bound - 1);
        assert_eq!(node.next.len(), level_bound);
        for link in node.next {
            assert_eq!(link, None);
        }
    }

    #[test]
    fn test_new() {
        let item = 42;
        let level = 3;
        let node = ZSkipNode::new(item, level);

        assert_eq!(node.val, Some(item));
        assert_eq!(node.level, level);
        assert_eq!(node.next.len(), level + 1);
        for link in node.next {
            assert_eq!(link, None);
        }
    }

    #[test]
    fn test_get_val() {
        let item = "Hello, world!";
        let node = ZSkipNode {
            val: Some(item),
            level: 2,
            next: vec![None, None, None],
        };

        assert_eq!(node.val, Some(item));
    }

    // Add more tests here...
}
