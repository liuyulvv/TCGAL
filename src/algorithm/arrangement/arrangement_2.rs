use std::{cell::RefCell, rc::Rc};

use crate::{
    algorithm::{
        arrangement::util::event_vertex_2::EventVertex2Type,
        intersection::edge_2_edge_2::edge_2_edge_2_intersection,
    },
    kernel::{edge_2::Edge2, face_2::Face2, number_type::NumberType, vertex_2::Vertex2},
};

use super::util::{
    event_queue::EventQueue, event_vertex_2::EventVertex2, status_structure::StatusStructure,
};

#[derive(Debug, Clone)]
pub struct Arrangement2<NT: NumberType> {
    vertices: Vec<Rc<RefCell<Vertex2<NT>>>>,
    edges: Vec<Rc<RefCell<Edge2<NT>>>>,
    faces: Vec<Rc<RefCell<Face2<NT>>>>,
    event_queue: EventQueue<NT>,
    status_structure: StatusStructure<NT>,
}

impl<NT: NumberType> Arrangement2<NT> {
    pub fn new(edges: Vec<Edge2<NT>>) -> Self {
        Self {
            vertices: Vec::new(),
            edges: edges
                .iter()
                .map(|e| Rc::new(RefCell::new(e.clone())))
                .collect(),
            faces: Vec::new(),
            event_queue: EventQueue::new(),
            status_structure: StatusStructure::new(),
        }
    }

    pub fn make_arrangement(&mut self) {
        self.event_queue = EventQueue::new();
        self.status_structure = StatusStructure::new();

        for edge in self.edges.iter() {
            self.event_queue.insert_edge(edge);
        }

        while !self.event_queue.is_empty() {
            let vertex = self.event_queue.pop();
            match vertex {
                Some(v) => {
                    let vertex_type = v.vertex_type();
                    match vertex_type {
                        EventVertex2Type::Start => {
                            println!(
                                "Start: {:?} {:?}",
                                v.vertex().borrow().x(),
                                v.vertex().borrow().y()
                            );
                            let p_edges = self.status_structure.get_p_edges(v.vertex().clone());
                            p_edges.l.iter().for_each(|e| {
                                self.status_structure.remove(e.clone());
                            });
                            p_edges.c.iter().for_each(|e| {
                                self.status_structure.remove(e.clone());
                            });
                            p_edges.u.iter().for_each(|e: &Rc<RefCell<Edge2<NT>>>| {
                                self.status_structure.insert(e.clone());
                            });
                            p_edges.c.iter().for_each(|e| {
                                self.status_structure.insert(e.clone());
                            });
                        }
                        EventVertex2Type::End => {
                            println!(
                                "End: {:?} {:?}",
                                v.vertex().borrow().x(),
                                v.vertex().borrow().y()
                            );
                            v.edges().iter().for_each(|e| {
                                let neighbors = self.status_structure.remove(e.clone());
                                match neighbors {
                                    Some(neighbors) => {
                                        self.process_neighbor(
                                            &v,
                                            neighbors.left.unwrap(),
                                            neighbors.right.unwrap(),
                                        );
                                    }
                                    None => {}
                                }
                            });
                        }
                        EventVertex2Type::Intersection => {
                            println!(
                                "Intersection: {:?} {:?}",
                                v.vertex().borrow().x(),
                                v.vertex().borrow().y()
                            );
                            let edges = v.edges();
                            let left = edges.get(0).unwrap();
                            let right = edges.get(1).unwrap();
                            self.swap_neighbor(&v, left.clone(), right.clone());
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    fn process_neighbor(
        &mut self,
        event_vertex: &EventVertex2<NT>,
        left: Rc<RefCell<Edge2<NT>>>,
        right: Rc<RefCell<Edge2<NT>>>,
    ) {
        let intersection = edge_2_edge_2_intersection(&left.borrow(), &right.borrow());
        intersection.iter().for_each(|point| {
            let vertex_binding = event_vertex.vertex();
            let vertex = vertex_binding.borrow();
            if point.y() < vertex.y() {
                self.event_queue.insert_intersection(
                    Rc::new(RefCell::new(Vertex2::new(point.x(), point.y()))),
                    left.clone(),
                    right.clone(),
                );
            } else if point.y().equals(vertex.y()) {
                match event_vertex.vertex_type() {
                    EventVertex2Type::Intersection => {
                        if point.x() > vertex.x() {
                            self.event_queue.insert_intersection(
                                Rc::new(RefCell::new(Vertex2::new(point.x(), point.y()))),
                                left.clone(),
                                right.clone(),
                            );
                        }
                    }
                    _ => {
                        if point.x() >= vertex.x() {
                            self.event_queue.insert_intersection(
                                Rc::new(RefCell::new(Vertex2::new(point.x(), point.y()))),
                                left.clone(),
                                right.clone(),
                            );
                        }
                    }
                }
            }
        });
    }

    fn swap_neighbor(
        &mut self,
        event_vertex: &EventVertex2<NT>,
        left: Rc<RefCell<Edge2<NT>>>,
        right: Rc<RefCell<Edge2<NT>>>,
    ) {
        if left.borrow().is_horizontal() || right.borrow().is_horizontal() {
            return;
        }
        let neighbors = self.status_structure.swap(left.clone(), right.clone());
        neighbors.iter().for_each(|neighbor| {
            self.process_neighbor(
                event_vertex,
                neighbor.left.clone().unwrap(),
                neighbor.right.clone().unwrap(),
            );
        });
    }

    pub fn vertices(&self) -> Vec<Rc<RefCell<Vertex2<NT>>>> {
        self.vertices.clone()
    }

    pub fn edges(&self) -> Vec<Rc<RefCell<Edge2<NT>>>> {
        self.edges.clone()
    }

    pub fn faces(&self) -> Vec<Rc<RefCell<Face2<NT>>>> {
        self.faces.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arrangement_2() {
        let mut edges = Vec::new();

        // let source = Vertex2::new(10.0, 10.0);
        // let target = Vertex2::new(0.0, 0.0);
        // let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        // edges.push(edge);

        // let source = Vertex2::new(0.0, 10.0);
        // let target = Vertex2::new(10.0, 0.0);
        // let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        // edges.push(edge);

        // let source = Vertex2::new(5.0, 0.0);
        // let target = Vertex2::new(8.0, 8.0);
        // let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        // edges.push(edge);

        let source = Vertex2::new(10.0, 10.0);
        let target = Vertex2::new(0.0, 10.0);
        let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        edges.push(edge);

        let source = Vertex2::new(0.0, 5.0);
        let target = Vertex2::new(5.0, 10.0);
        let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        edges.push(edge);

        let source = Vertex2::new(3.0, 0.0);
        let target = Vertex2::new(3.0, 15.0);
        let edge = Edge2::new_segment(Rc::new(RefCell::new(source)), Rc::new(RefCell::new(target)));
        edges.push(edge);

        let mut arrangement = Arrangement2::new(edges);
        arrangement.make_arrangement();
    }
}
