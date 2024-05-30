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

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
