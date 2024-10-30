struct MyVec<T> {
    ptr: *mut T,
    size: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    const CAPACITY_INCREMENTS: usize = 4;

    fn new() -> Self {
        todo!()
    }

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn reserve(&mut self, additional: usize) {
        todo!()
    }

    fn push(&mut self, value: T) {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn get(&self, index: usize) -> Option<T> {
        todo!()
    }

    fn expand_capacity(&mut self) {
        todo!()
    }

    unsafe fn pointer_to_elem(&self, index: usize) -> *mut T {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let mut my_vec = MyVec::new();
        my_vec.push(0);
        assert_eq!(my_vec.size, 1);
        assert_eq!(my_vec.capacity, 4);
        assert_eq!(my_vec.pop(), Some(0));
        assert_eq!(my_vec.pop(), None);

        for i in 0..10 {
            my_vec.push(i);
            assert_eq!(Some(i), my_vec.get(i));
        }

        for i in 0..10 {
            assert_eq!(Some(10 - i - 1), my_vec.pop())
        }

        my_vec.push(17);
        my_vec.push(38);
        my_vec.push(42);
        my_vec.push(314);
        my_vec.push(1);

        assert_eq!(Some(1), my_vec.pop());
        assert_eq!(Some(314), my_vec.pop());
        assert_eq!(Some(42), my_vec.pop());
        assert_eq!(Some(38), my_vec.pop());
        assert_eq!(Some(17), my_vec.pop());
        assert_eq!(None, my_vec.pop());
        assert_eq!(None, my_vec.pop());
        assert_eq!(None, my_vec.pop());
        assert_eq!(None, my_vec.pop());
    }
}
