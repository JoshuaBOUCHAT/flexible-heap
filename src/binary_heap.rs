use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Fn, Index, IndexMut};

#[derive(Debug, Clone)]
pub struct BinaryHeap<T, F: Fn(&T, &T) -> Ordering> {
    data: Vec<T>,
    func: F,
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> BinaryHeap<T, F> {
    pub fn from_array(data: Vec<T>, func: F) -> Self {
        let mut heap = BinaryHeap { data, func };
        if heap.len() > 1 {
            let first_index = (heap.len() >> 1) - 1;
            for i in (0..first_index).rev() {
                heap.heapify(i);
            }
        }

        heap
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    fn heapify(&mut self, root: usize) {
        let right = (root + 1) << 1;
        let left = right - 1;
        if left >= self.len() {
            return;
        }

        let index = if right < self.len() && self(right, left) == Ordering::Greater {
            right
        } else {
            left
        };
        if self(index, root) == Ordering::Greater {
            self.data.swap(root, index);
            self.heapify(index);
        }
    }
    pub fn peak(&self) -> Option<&T> {
        self.data.get(0)
    }
}

impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> FnOnce<(usize, usize)> for BinaryHeap<T, F> {
    type Output = Ordering;
    extern "rust-call" fn call_once(self, args: (usize, usize)) -> Self::Output {
        (self.func)(&self.data[args.0], &self.data[args.1])
    }
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> Fn<(usize, usize)> for BinaryHeap<T, F> {
    extern "rust-call" fn call(&self, args: (usize, usize)) -> Self::Output {
        (self.func)(&self.data[args.0], &self.data[args.1])
    }
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> FnMut<(usize, usize)> for BinaryHeap<T, F> {
    extern "rust-call" fn call_mut(&mut self, args: (usize, usize)) -> Self::Output {
        (self.func)(&self.data[args.0], &self.data[args.1])
    }
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> Index<usize> for BinaryHeap<T, F> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> IndexMut<usize> for BinaryHeap<T, F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
