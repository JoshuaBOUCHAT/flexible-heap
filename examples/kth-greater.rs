use flexible_heap;
#[derive(Clone, Debug, Copy)]
struct Complex {
    x: f32,
    y: f32,
}
impl Complex {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
/// This main ilustrate how to get the first k complexs with the lowest norm
fn main() {
    let mut heap = flexible_heap::BinaryHeap::new(|a: &Complex, b: &Complex| {
        (b.norm()).partial_cmp(&a.norm()).unwrap()
    });
    heap.push(Complex::new(0., 0.));
    heap.push(Complex::new(1., 1.));
    heap.push(Complex::new(3., 0.));
    heap.push(Complex::new(5., -8.));
    let k = 2;
    for _ in 0..k {
        println!("{:?}", heap.pop())
    }
}
