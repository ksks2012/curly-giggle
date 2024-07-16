use curly_giggle::collection::skiplist::zskiplist::ZSkipList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zsl_create() {
        let list: ZSkipList<i32> = ZSkipList::zsl_create();
        assert_eq!(list.get_len(), 0);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut list = ZSkipList::zsl_create();

        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);

        assert_eq!(list.contains(&1), true);
        assert_eq!(list.contains(&2), true);
        assert_eq!(list.contains(&3), true);
        assert_eq!(list.contains(&4), false);
    }

    #[test]
    fn test_zsl_delete() {
        let mut list = ZSkipList::zsl_create();
        
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);

        assert_eq!(list.zsl_delete(&2), Some(2));
        assert_eq!(list.zsl_delete(&2), None);
        assert_eq!(list.contains(&2), false);
    }

    #[test]
    fn test_iter() {
        let mut list: ZSkipList<i32> = ZSkipList::zsl_create();
        for i in 0..100 {
            list.zsl_insert(1.0, i);
        }

        let mut x = 0;
        list.iter().for_each(|i| {
            assert_eq!(i, &x);
            x += 1;
        });

    }

    #[test]
    fn test_iter_mut() {
        let mut list = ZSkipList::zsl_create();
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);
        
        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), None);
    }

    #[test]
    fn test_empty_list() {
        let mut list = ZSkipList::zsl_create();
        assert_eq!(list.contains(&1), false);
        assert_eq!(list.zsl_delete(&1), None);
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_duplicates() {
        let mut list = ZSkipList::zsl_create();
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);

        assert_eq!(list.contains(&1), true);
        assert_eq!(list.zsl_delete(&1), Some(1));
        assert_eq!(list.get_len(), 2);
        assert_eq!(list.contains(&1), false);
        assert_eq!(list.contains(&2), true);
        assert_eq!(list.contains(&3), true);
        
        assert_eq!(list.zsl_delete(&1), None);
        assert_eq!(list.zsl_delete(&2), Some(2));
        assert_eq!(list.contains(&2), false);

        assert_eq!(list.zsl_delete(&3), Some(3));
        assert_eq!(list.contains(&3), false);
    }

    #[test]
    fn test_zsl_get_rank() {
        let mut list = ZSkipList::zsl_create();
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);

        assert_eq!(list.zsl_get_rank(1.0, 1), Some(0));
        assert_eq!(list.zsl_get_rank(2.0, 2), Some(1));
        assert_eq!(list.zsl_get_rank(3.0, 3), Some(2));
        assert_eq!(list.zsl_get_rank(4.0, 4), None);
    }

    #[test]
    fn test_zsl_get_element_by_rank() {
        let mut list = ZSkipList::zsl_create();
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);

        assert_eq!(list.zsl_get_element_by_rank(0), None);
        assert_eq!(list.zsl_get_element_by_rank(1), Some(1));
        assert_eq!(list.zsl_get_element_by_rank(2), Some(2));
        assert_eq!(list.zsl_get_element_by_rank(3), Some(3));
        assert_eq!(list.zsl_get_element_by_rank(4), None);
    }

    #[test]
    fn test_zsl_is_in_range() {
        let mut list = ZSkipList::zsl_create();
        list.zsl_insert(1.0, 1);
        list.zsl_insert(2.0, 2);
        list.zsl_insert(3.0, 3);
        
        assert_eq!(list.zsl_is_in_range(1.0, 3.0), true);
        assert_eq!(list.zsl_is_in_range(2.0, 4.0), true);
        assert_eq!(list.zsl_is_in_range(0.0, 0.9), false);
        assert_eq!(list.zsl_is_in_range(4.0, 5.0), false);
    }

    // Add more tests here...
}