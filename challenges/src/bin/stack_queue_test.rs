#[cfg(test)]
mod tests {
    use challenges::stack_queue::StackQueue;
    use std::iter::FromIterator;

    #[test]
    fn new_queue_is_empty() {
        let queue: StackQueue<i32, 5> = StackQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn push_and_pop() {
        let mut queue = StackQueue::<i32, 5>::new();
        assert_eq!(queue.push(1), Ok(()));
        assert_eq!(queue.push(2), Ok(()));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert!(queue.is_empty());
    }

    #[test]
    fn front_and_back() {
        let mut queue = StackQueue::<i32, 5>::new();
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        assert_eq!(queue.front(), Some(&1));
        assert_eq!(queue.back(), Some(&2));
    }

    #[test]
    fn full_queue() {
        let mut queue = StackQueue::<i32, 2>::new();
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        assert!(queue.is_full());
        assert_eq!(queue.push(3), Err("Queue is full".to_string()));
    }

    #[test]
    fn clear_queue() {
        let mut queue = StackQueue::<i32, 5>::new();
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn from_iterator() {
        let queue = StackQueue::<i32, 5>::from_iter(vec![1, 2, 3]);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn extend_queue() {
        let mut queue = StackQueue::<i32, 5>::new();
        queue.extend(vec![1, 2, 3]);
        assert_eq!(queue.len(), 3);
    }
}
