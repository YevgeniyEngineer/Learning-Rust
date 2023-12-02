use std::fmt;
use std::iter::DoubleEndedIterator;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;
use std::slice;

pub struct StackVector<T, const MAX_SIZE: usize> {
    data: [mem::MaybeUninit<T>; MAX_SIZE],
    size: usize,
}

impl<T, const MAX_SIZE: usize> StackVector<T, MAX_SIZE> {
    pub fn new() -> Self {
        Self {
            data: unsafe { mem::MaybeUninit::uninit().assume_init() },
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, value: T) {
        if self.size >= MAX_SIZE {
            panic!("Capacity exceeded");
        }
        unsafe {
            ptr::write(self.data[self.size].as_mut_ptr(), value);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe { Some(ptr::read(self.data[self.size].as_ptr())) }
        }
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T, const MAX_SIZE: usize> Drop for StackVector<T, MAX_SIZE> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: Clone, const MAX_SIZE: usize> Clone for StackVector<T, MAX_SIZE> {
    fn clone(&self) -> Self {
        let mut new_vector = StackVector::new();
        for i in 0..self.size {
            new_vector.push(unsafe { self.data[i].assume_init_ref() }.clone());
        }
        new_vector
    }
}

impl<T: fmt::Debug, const MAX_SIZE: usize> fmt::Debug for StackVector<T, MAX_SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, const MAX_SIZE: usize> Index<usize> for StackVector<T, MAX_SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { self.data[index].assume_init_ref() }
    }
}

impl<T, const MAX_SIZE: usize> IndexMut<usize> for StackVector<T, MAX_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { self.data[index].assume_init_mut() }
    }
}

pub struct StackVectorIntoIterator<T, const MAX_SIZE: usize> {
    vector: StackVector<T, MAX_SIZE>,
    index: usize,
}

impl<T, const MAX_SIZE: usize> IntoIterator for StackVector<T, MAX_SIZE> {
    type Item = T;
    type IntoIter = StackVectorIntoIterator<T, MAX_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        StackVectorIntoIterator {
            vector: self,
            index: 0,
        }
    }
}

impl<T, const MAX_SIZE: usize> Iterator for StackVectorIntoIterator<T, MAX_SIZE> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.size {
            let value = unsafe { ptr::read(self.vector.data[self.index].as_ptr()) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T, const MAX_SIZE: usize> StackVector<T, MAX_SIZE> {
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        unsafe {
            // SAFETY: Assuming elements are initialized up to `self.size`
            let slice = slice::from_raw_parts(self.data.as_ptr() as *const T, self.size);
            slice.iter()
        }
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        unsafe {
            // SAFETY: Assuming elements are initialized up to `self.size`
            let slice = slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.size);
            slice.iter_mut()
        }
    }
}

impl<T, const MAX_SIZE: usize> StackVector<T, MAX_SIZE> {
    pub fn iter_rev(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.iter().rev()
    }

    pub fn iter_mut_rev(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.iter_mut().rev()
    }
}

impl<T: PartialEq, const MAX_SIZE: usize> PartialEq for StackVector<T, MAX_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq, const MAX_SIZE: usize> Eq for StackVector<T, MAX_SIZE> {}

impl<T, const MAX_SIZE: usize> Default for StackVector<T, MAX_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
