use std::fmt;
use std::iter::{Extend, FromIterator};
use std::mem::MaybeUninit;

pub struct StackQueue<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    front: usize,
    rear: usize,
    size: usize,
}

impl<T, const N: usize> StackQueue<T, N> {
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= N {
            Err("Queue is full".to_string())
        } else {
            // SAFETY: The container is guaranteed to have space
            unsafe {
                self.data[self.rear].as_mut_ptr().write(item);
            }
            self.rear = (self.rear + 1) % N;
            self.size += 1;
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            // SAFETY: The container is guaranteed to have an element
            let item = unsafe { self.data[self.front].as_mut_ptr().read() };
            self.front = (self.front + 1) % N;
            self.size -= 1;
            Some(item)
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            // SAFETY: The front element is guaranteed to be initialized
            unsafe { Some(&*self.data[self.front].as_ptr()) }
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            None
        } else {
            // SAFETY: The front element is guaranteed to be initialized
            unsafe { Some(&mut *self.data[self.front].as_mut_ptr()) }
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            // SAFETY: The last element is guaranteed to be initialized
            let last_index = if self.rear == 0 { N - 1 } else { self.rear - 1 };
            unsafe { Some(&*self.data[last_index].as_ptr()) }
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            None
        } else {
            // SAFETY: The last element is guaranteed to be initialized
            let last_index = if self.rear == 0 { N - 1 } else { self.rear - 1 };
            unsafe { Some(&mut *self.data[last_index].as_mut_ptr()) }
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.size == N
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.front = 0;
        self.rear = 0;
        self.size = 0;
    }
}

impl<T, const N: usize> Drop for StackQueue<T, N> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for StackQueue<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut elements = Vec::with_capacity(self.size);
        for i in 0..self.size {
            let index = (self.front + i) % N;
            unsafe {
                elements.push(&*self.data[index].as_ptr());
            }
        }
        f.debug_list().entries(elements).finish()
    }
}

impl<T, const N: usize> FromIterator<T> for StackQueue<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut queue = Self::new();
        for item in iter {
            if queue.push(item).is_err() {
                // Stop if the queue is full
                break;
            }
        }
        queue
    }
}

impl<T, const N: usize> Extend<T> for StackQueue<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            if self.push(item).is_err() {
                // Stop if the queue is full
                break;
            }
        }
    }
}
