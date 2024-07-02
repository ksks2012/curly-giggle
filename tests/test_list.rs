use curly_giggle::collection::list::LinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);

        let front_elem = *list.front().unwrap();
        assert_eq!(front_elem, 3);
        
        let back_elem = *list.back().unwrap();
        assert_eq!(back_elem, 1);
    }

    #[test]
    fn test_pop_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let popped_elem = list.pop_front();
        assert_eq!(popped_elem, Some(3));
        assert_eq!(list.len(), 2);

        let front_elem = *list.front().unwrap();
        assert_eq!(front_elem, 2);

        let back_elem = *list.back().unwrap();
        assert_eq!(back_elem, 1);
    }
}
