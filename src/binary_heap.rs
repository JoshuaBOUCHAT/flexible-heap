use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Fn, Index, IndexMut};

#[derive(Debug, Clone)]
pub struct BinaryHeap<T, F: Fn(&T, &T) -> Ordering> {
    data: Vec<T>,
    func: F,
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> BinaryHeap<T, F> {
    pub fn new(func: F) -> Self {
        BinaryHeap { data: vec![], func }
    }
    fn compare(&self, x: usize, y: usize) -> Ordering {
        (self.func)(&self.data[x], &self.data[y])
    }

    pub fn from_array(data: Vec<T>, func: F) -> Self {
        let mut heap = BinaryHeap { data, func };
        if heap.len() > 1 {
            let first_index = (heap.len() >> 1) - 1;
            for i in (0..=first_index).rev() {
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

        let index = if right < self.len() && self.compare(right, left) == Ordering::Greater {
            right
        } else {
            left
        };
        if self.compare(index, root) == Ordering::Greater {
            self.data.swap(root, index);
            self.heapify(index);
        }
    }
    pub fn peak(&self) -> Option<&T> {
        self.data.get(0)
    }
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let last = self.len() - 1;
        self.data.swap(0, last);
        let temp = self.data.pop();
        if self.len() > 1 {
            self.heapify(0);
        }

        temp
    }
    pub fn push(&mut self, item: T) {
        let mut index = self.len();
        self.data.push(item);

        while index > 0 {
            let parent = (index - 1) >> 1;
            if self.compare(parent, index) == Ordering::Greater {
                break;
            }
            self.data.swap(parent, index);
            index = parent;
        }
    }
    pub fn update_top(&mut self, item: T) {
        if self.len() == 0 {
            self.push(item);
            return;
        }
        if (self.func)(&self[0], &item) == Ordering::Greater {
            self[0] = item;
            self.heapify(0);
        } else {
            self[0] = item;
        }
    }
    pub fn extend(&mut self, items: &[T]) {
        for item in items {
            self.push(item.clone());
        }
    }
}
impl<T: Debug + Clone + Ord, F: Fn(&T, &T) -> Ordering> From<Vec<T>> for BinaryHeap<T, F> {
    fn from(data: Vec<T>) -> Self {
        BinaryHeap { data, func: T::cmp }
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
#[cfg(test)]
mod test {
    use crate::BinaryHeap;
    #[test]
    fn test_heapify() {
        let original = vec![5, 4, 3, 2, 1];
        let heap = BinaryHeap::from_array(original, |a, b| b.cmp(a));
        //[5, 4, 3, 2, 1]
        //[5, 1, 3, 2, 4]
        //[1, 5, 3, 2, 4]
        //[1, 2 ,3 ,5 ,4]

        let inner = heap.into_inner();
        assert!(inner.as_slice() == [1, 2, 3, 5, 4].as_slice())
    }
    #[test]
    fn test_pop() {
        let mut heap = BinaryHeap::from_array(vec![5, 4, 3, 2, 1], |a, b| b.cmp(a));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }
    #[test]
    fn test_push() {
        let mut heap = BinaryHeap::new(|a: &i32, b: &i32| a.cmp(b));
        heap.push(10i32);
        heap.push(3i32);
        heap.push(128i32);
        assert_eq!(heap.pop(), Some(128));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }
    #[test]
    fn test_update() {
        let mut heap = BinaryHeap::from_array(vec![5, 4, 3, 2, 1], |a, b| b.cmp(a));
        heap.update_top(6);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), None);
    }
}
