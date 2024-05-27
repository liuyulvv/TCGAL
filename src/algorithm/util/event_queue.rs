use std::collections::BinaryHeap;

use crate::{
    kernel::base_dcel::base_edge_2::BaseEdge2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::event_vertex_2::{EventVertex2, EventVertex2Type};

pub struct EventQueue<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> {
    events: BinaryHeap<EventVertex2<'a, NT, T>>,
}

impl<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> EventQueue<'a, NT, T> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn insert_edge(&mut self, edge: &'a T) {
        let mut source = EventVertex2::new(edge.source(), EventVertex2Type::Start);
        source.add_edge(edge);
        let end = EventVertex2::new(edge.target(), EventVertex2Type::End);
        self.events.push(source);
        self.events.push(end);
    }

    pub fn pop(&mut self) -> Option<EventVertex2<'a, NT, T>> {
        self.events.pop()
    }
}

#[cfg(test)]
mod tests {

    use crate::kernel::{
        base_dcel::base_vertex_2::BaseVertex2,
        simple_cartesian::{edge_2::Edge2, vertex_2::Vertex2},
    };

    use super::*;

    #[test]
    fn test_insert_edge() {
        let mut event_queue = EventQueue::new();
        let vertex_start = Vertex2::new(0.0, 0.0);
        let vertex_end = Vertex2::new(1.0, 1.0);
        let edge = Edge2::new_segment(&vertex_start, &vertex_end);
        event_queue.insert_edge(&edge);
        assert_eq!(event_queue.events.len(), 2);
        let start = event_queue.pop().unwrap();
        let end = event_queue.pop().unwrap();
        start.edges().iter().for_each(|e| {
            assert_eq!(e.source().equals(&vertex_start), true);
            assert_eq!(e.target().equals(&vertex_end), true);
        });
        end.edges().iter().for_each(|e| {
            assert_eq!(e.source().equals(&vertex_start), true);
            assert_eq!(e.target().equals(&vertex_end), true);
        });
        assert_eq!(start.vertex_type(), &EventVertex2Type::End);
        assert_eq!(end.vertex_type(), &EventVertex2Type::Start);
    }
}
