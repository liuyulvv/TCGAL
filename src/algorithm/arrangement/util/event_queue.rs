use std::{cell::RefCell, collections::BinaryHeap, rc::Rc};

use crate::kernel::{edge_2::Edge2, number_type::NumberType, vertex_2::Vertex2};

use super::event_vertex_2::{EventVertex2, EventVertex2Type};

#[derive(Debug, Clone)]
pub struct EventQueue<NT: NumberType> {
    events: BinaryHeap<EventVertex2<NT>>,
}

impl<NT: NumberType> EventQueue<NT> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    pub fn insert_edge(&mut self, edge: &Rc<RefCell<Edge2<NT>>>) {
        let edge_binding = edge.borrow();

        let source = edge_binding.source();
        let source: Option<&EventVertex2<NT>> = self.find(&source);
        match source {
            Some(e) => {
                let mut event = EventVertex2::new(edge_binding.source(), EventVertex2Type::Start);
                e.edges().iter().for_each(|e| {
                    event.add_edge(e.clone());
                });
                event.add_edge(edge.clone());
                self.remove(&e.vertex());
                self.events.push(event);
            }
            None => {
                let mut event = EventVertex2::new(edge_binding.source(), EventVertex2Type::Start);
                event.add_edge(edge.clone());
                self.events.push(event);
            }
        }

        let target = edge_binding.target();
        let target = self.find(&target);
        match target {
            Some(e) => {
                let mut event = EventVertex2::new(edge_binding.target(), EventVertex2Type::End);
                e.edges().iter().for_each(|e| {
                    event.add_edge(e.clone());
                });
                event.add_edge(edge.clone());
                self.remove(&e.vertex());
                self.events.push(event);
            }
            None => {
                let mut event = EventVertex2::new(edge_binding.target(), EventVertex2Type::End);
                event.add_edge(edge.clone());
                self.events.push(event);
            }
        }
    }

    pub fn insert_intersection(&mut self, vertex: Rc<RefCell<Vertex2<NT>>>) {
        self.events
            .push(EventVertex2::new(vertex, EventVertex2Type::Intersection));
    }

    pub fn pop(&mut self) -> Option<EventVertex2<NT>> {
        self.events.pop()
    }

    pub fn find(&self, vertex: &Rc<RefCell<Vertex2<NT>>>) -> Option<&EventVertex2<NT>> {
        self.events.iter().find(|e| {
            let binding = e.vertex();
            let e_vertex = binding.borrow();
            let v_vertex = vertex.borrow();
            e_vertex.equals(&v_vertex)
        })
    }

    pub fn remove(&mut self, vertex: &Rc<RefCell<Vertex2<NT>>>) {
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
