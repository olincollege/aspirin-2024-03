use std::alloc::{alloc, dealloc, realloc, Layout};

struct MyVec<T> {
    ptr: *mut T,
    size: usize,
    capacity: usize,
}

fn foo() {
    let mut a = Box::new(100);
    let b = Box::new(200);

    // a = b;
    drop(a);
    a = b;
}

impl<T> MyVec<T> {
    const CAPACITY_INCREMENTS: usize = 4;

    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        let mut new_instance = Self {
            ptr: std::ptr::null_mut(),
            size: 0,
            capacity: capacity,
        };

        new_instance.expand_capacity(capacity);

        new_instance
    }

    fn reserve(&mut self, additional: usize) {
        let new_capacity = self
            .size
            .checked_add(additional)
            .expect("vector capacity overflowed");

        let old_layout = Layout::array::<T>(self.capacity).expect("old arr layout");
        let new_layout = Layout::array::<T>(new_capacity).expect("new arr layout");

        unsafe {
            let new_ptr = realloc(self.ptr as _, old_layout, new_layout.size());
            self.ptr = new_ptr as *mut T;
            self.capacity = new_capacity;
        }
    }

    fn push(&mut self, value: T) {
        // Ensure that the capacity of the vector is sufficient to add another
        // value
        if self.capacity == self.size {
            self.expand_capacity(
                self.capacity
                    .checked_add(Self::CAPACITY_INCREMENTS)
                    .expect("capacity overflowed"),
            );
        }

        // SAFETY: Pointer should be within the bounds of allocation because of the previous
        // capacity expansion check
        unsafe {
            let new_elem_ptr = self.pointer_to_elem(self.size);
            new_elem_ptr.write(value);
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            // SAFETY: due to the allocation with this module, we are certain
            // that there will be memory at the specific offset to the ptr
            let pop_elem_ptr = unsafe { self.pointer_to_elem(self.size) };

            Some(unsafe { pop_elem_ptr.read() })
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if self.size == 0 || index >= self.size {
            None
        } else {
            Some(unsafe { self.pointer_to_elem(index).as_ref()? })
        }
    }

    fn expand_capacity(&mut self, new_capacity: usize) {
        if self.capacity == 0 {
            let layout = Layout::array::<T>(new_capacity).expect("size is four");
            // SAFETY: layout will never have a size of 0
            let initial_ptr = unsafe { alloc(layout) };
            self.ptr = initial_ptr as _;
        } else {
            let old_layout =
                Layout::array::<T>(self.capacity).expect("already created this layout");
            let new_layout =
                Layout::array::<T>(new_capacity).expect("already checked for no overflow");

            // SAFETY: verified that the ptr is the new, old layout was used for previous
            // allocation, new size must be greater than zero since capacity has incremented
            let new_ptr = unsafe { realloc(self.ptr as _, old_layout, new_layout.size()) };
            self.ptr = new_ptr as _;
            self.capacity = new_capacity;
        }
    }

    /// SAFETY: index must be non-zero and not overflow the increment to `self.ptr`
    unsafe fn pointer_to_elem(&self, index: usize) -> *mut T {
        self.ptr.add(index)
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.size == 0 {
            return;
        }

        unsafe {
            for index in 0..self.size {
                std::ptr::drop_in_place(self.pointer_to_elem(index))
            }
        }

        let layout = Layout::array::<T>(self.capacity).expect("layout has already been allocated");

        unsafe {
            dealloc(self.ptr as _, layout);
        }
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
            assert_eq!(Some(&i), my_vec.get(i));
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
