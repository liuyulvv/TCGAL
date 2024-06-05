use std::collections::BinaryHeap;

pub struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}
