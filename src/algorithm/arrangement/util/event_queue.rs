use std::{cell::RefCell, collections::BinaryHeap, rc::Rc};

use crate::kernel::{edge_2::Edge2, number_type::NumberType, vertex_2::Vertex2};

use super::event_vertex_2::{EventVertex2, EventVertex2Type};

#[derive(Debug, Clone)]
pub struct EventQueue<T: NumberType> {
    events: BinaryHeap<EventVertex2<T>>,
}

impl<T: NumberType> EventQueue<T> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn insert_edge(&mut self, edge: &Rc<RefCell<Edge2<T>>>) {
        let edge_binding = edge.borrow();

        let mut event = EventVertex2::new(edge_binding.source(), EventVertex2Type::Start);
        event.add_edge(edge.clone());
        self.events.push(event);

        let mut event = EventVertex2::new(edge_binding.target(), EventVertex2Type::End);
        event.add_edge(edge.clone());
        self.events.push(event);
    }

    pub fn insert_intersection(
        &mut self,
        vertex: Rc<RefCell<Vertex2<T>>>,
        left_edge: Rc<RefCell<Edge2<T>>>,
        right_edge: Rc<RefCell<Edge2<T>>>,
    ) {
        let mut event = EventVertex2::new(vertex.clone(), EventVertex2Type::Intersection);
        event.add_edge(left_edge.clone());
        event.add_edge(right_edge.clone());
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<EventVertex2<T>> {
        self.events.pop()
    }

    pub fn remove(&mut self, vertex: &Rc<RefCell<Vertex2<T>>>) {
        self.events.retain(|e| {
            let binding = e.vertex();
            let e_vertex = binding.borrow();
            let v_vertex = vertex.borrow();
            !e_vertex.equals(&v_vertex)
        });
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
