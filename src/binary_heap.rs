use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Fn, Index, IndexMut};

/// A flexible implementation of a BinaryHeap,
/// flexible in the sens of you don't need the type of the object Ord or PartialOrd you just haveto provide a function to compare all the elements.
/// This heap is max-heap if you want a min heap you just need to reverse the orderer inside the function you pass.
/// Transform an array into a heap is O(n), push and pop are O(log(n))
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
    ///Create a binary heap by taking ownership of the array. This operation is done in O(N) time.
    pub fn from_array(data: Vec<T>, func: F) -> Self {
        let mut heap = BinaryHeap { data, func };
        heap.heapify();
        heap
    }
    fn heapify(&mut self) {
        if self.len() > 1 {
            let first_index = (self.len() >> 1) - 1;
            for i in (0..=first_index).rev() {
                self.sift_down(i);
            }
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    fn sift_down(&mut self, mut root: usize) {
        loop {
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
                root = index;
            } else {
                return;
            }
        }
    }
    ///Return a reference of the element on the top of the heap
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
            self.sift_down(0);
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
    ///Change the value of the root node if the function return Ord::Greater the elements if Copied O(1) but it's smaller than we need to performe a sift_dowm O(log(n))
    pub fn update_top(&mut self, item: T) {
        if self.len() == 0 {
            self.push(item);
            return;
        }
        if (self.func)(&self[0], &item) == Ordering::Greater {
            self[0] = item;
            self.sift_down(0);
        } else {
            self[0] = item;
        }
    }
    /// This function add each elements one by one with log(n) each push -> k*log(n+k) where k is the number of elements to add.
    /// If you want to add a big  number of item comparer to what's already inside the heap you should consider using bulk_extend.
    pub fn extend(&mut self, items: &[T]) {
        for item in items {
            self.push(item.clone());
        }
    }
    /// This function is use when inserting a big amout of items into an existing heap if the number of elements is small prefer using the normal extend
    pub fn bulk_extend(&mut self, items: &[T]) {
        self.data.extend_from_slice(items);
        self.heapify();
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
///Take ownership of the BinaryHeap and return an iterator of sorted elements
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> IntoIterator for BinaryHeap<T, F> {
    type Item = T;
    type IntoIter = HeapIterator<T, F>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { inner: self }
    }
}

pub struct HeapIterator<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> {
    inner: BinaryHeap<T, F>,
}
impl<T: Debug + Clone, F: Fn(&T, &T) -> Ordering> Iterator for HeapIterator<T, F> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

#[cfg(test)]
mod test {
    use std::f64::INFINITY;

    use crate::BinaryHeap;
    #[test]
    fn test_sift_down() {
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
    #[test]
    fn test_iterator() {
        let mut heap = BinaryHeap::from_array(vec![5, 4, 3, 2, 1], |a, b| b.cmp(a));
        heap.update_top(6);
        assert_eq!(heap.into_iter().collect::<Vec<i32>>(), vec![2, 3, 4, 5, 6]);
    }
    #[test]
    fn test_float() {
        let heap = BinaryHeap::from_array(vec![0.0, 1.3, -107.18, INFINITY], |a: &f64, b: &f64| {
            a.partial_cmp(&b).unwrap()
        });
        for float in heap.into_iter() {
            println!("{float}")
        }
    }
}
