use crate::kernel::{number_type::NumberType, point_2::Point2};
use std::collections::BinaryHeap;

pub struct SweepEventQueue<T: NumberType> {
    heap: BinaryHeap<Point2<T>>,
}

impl<T: NumberType> SweepEventQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, value: Point2<T>) {
        self.heap.push(value);
    }

    pub fn pop(&mut self) -> Option<Point2<T>> {
        self.heap.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}
