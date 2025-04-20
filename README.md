# flexible-heap

A very simple implementation of a binary heap that uses a function passed as a parameter to compare the items. This makes it very simple to use with non-Ord items, as you only need to provide a comparing function.
A simple example is if you want to implement a binary heap on floats like f64. In the standard library, you won't be able to because f64 doesn't implement Ord, only PartialOrd. You can easily create a flexible heap in this way:
```rust
let heap = BinaryHeap::from_array(vec![0.0, 1.3, -107.18, INFINITY], |a: &f64, b: &f64| {
  a.partial_cmp(&b).unwrap()
});

//into_iter iterates over sorted items; each element needs log(n) to be popped, this is done in n*log(n)
for float in heap.into_iter() {
  println!("{float}")
}
```
