# flexible-heap

A very simple implementation of a binary heap that use a function passed by parameter to compare the items. This make it very simple to use on non Ord item as you only need to provide a comparing function.
A simple exemple is if you wanty to impl a binary heap on floats like f64. In the std you wont be able because f64 doesn't impl Ord, only PartialOrd. You can easly create a flexible one this way:
```rust
let heap = BinaryHeap::from_array(vec![0.0, 1.3, -107.18, INFINITY], |a: &f64, b: &f64| {
  a.partial_cmp(&b).unwrap()
});

//into_iter iter over sorted items each element needing log(n) to be pop this is done in nlog(n)
for float in heap.into_iter() {
  println!("{float}")
}
```
