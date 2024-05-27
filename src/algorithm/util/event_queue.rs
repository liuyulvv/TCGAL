use std::collections::BinaryHeap;

use crate::{
    kernel::base_dcel::base_edge_2::BaseEdge2, kernel::base_dcel::base_vertex_2::BaseVertex2,
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
        let source = self.find(edge.source());
        match source {
            Some(e) => {
                let mut event = EventVertex2::new(edge.source(), EventVertex2Type::Start);
                e.edges().iter().for_each(|e| {
                    event.add_edge(*e);
                });
                event.add_edge(edge);
                self.remove(e.vertex());
                self.events.push(event);
            }
            None => {
                let mut event = EventVertex2::new(edge.source(), EventVertex2Type::Start);
                event.add_edge(edge);
                self.events.push(event);
            }
        }

        let target = self.find(edge.target());
        match target {
            Some(e) => {
                let mut event = EventVertex2::new(edge.target(), EventVertex2Type::End);
                e.edges().iter().for_each(|e| {
                    event.add_edge(*e);
                });
                event.add_edge(edge);
                self.remove(e.vertex());
                self.events.push(event);
            }
            None => {
                let mut event = EventVertex2::new(edge.target(), EventVertex2Type::End);
                event.add_edge(edge);
                self.events.push(event);
            }
        }
    }

    pub fn pop(&mut self) -> Option<EventVertex2<'a, NT, T>> {
        self.events.pop()
    }

    pub fn find(&self, vertex: &T::Vertex) -> Option<&EventVertex2<'a, NT, T>> {
        self.events.iter().find(|e| e.vertex().equals(vertex))
    }

    pub fn remove(&mut self, vertex: &T::Vertex) {
        self.events.retain(|e| !e.vertex().equals(vertex));
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

        event_queue.insert_edge(&edge);
        assert_eq!(event_queue.events.len(), 2);

        let start = event_queue.find(&vertex_start);
        assert_eq!(start.is_some(), true);
        let start = start.unwrap();
        assert_eq!(start.edges().len(), 2);

        let end = event_queue.find(&vertex_end);
        assert_eq!(end.is_some(), true);
        let end = end.unwrap();
        assert_eq!(end.edges().len(), 2);

        // let vertex_start = Vertex2::new(0.0, 0.0);
        // let vertex_end = Vertex2::new(1.0, 1.0);
        // let edge = Edge2::new_segment(&vertex_start, &vertex_end);
    }
}
