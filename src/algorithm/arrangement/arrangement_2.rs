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
                            v.edges().iter().for_each(|e| {
                                let neighbors = self.status_structure.insert(e.clone());
                                match neighbors {
                                    Some(neighbors) => {
                                        match neighbors.left {
                                            Some(left) => {
                                                self.process_neighbor(&v, e.clone(), left)
                                            }
                                            None => {}
                                        }
                                        match neighbors.right {
                                            Some(right) => {
                                                self.process_neighbor(&v, e.clone(), right)
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            });
                        }
                        EventVertex2Type::End => {
                            println!(
                                "End: {:?} {:?}",
                                v.vertex().borrow().x(),
                                v.vertex().borrow().y()
                            );
                            v.edges().iter().for_each(|e| {
                                self.status_structure.remove(e.clone());
                            });
                        }
                        EventVertex2Type::Intersection => {
                            println!(
                                "Intersection: {:?} {:?}",
                                v.vertex().borrow().x(),
                                v.vertex().borrow().y()
                            );
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
        edge: Rc<RefCell<Edge2<NT>>>,
        neighbor: Rc<RefCell<Edge2<NT>>>,
    ) {
        todo!()
        // let intersection = edge_2_edge_2_intersection(&neighbor.borrow(), &edge.borrow());
        // intersection.iter().for_each(|point| {
        //     let vertex_binding = event_vertex.vertex();
        //     let vertex = vertex_binding.borrow();
        //     if point.y() < vertex.y() || (point.y().equals(vertex.y()) && point.x() >= vertex.x()) {
        //         self.event_queue
        //             .insert_intersection(Rc::new(RefCell::new(Vertex2::new(point.x(), point.y()))));
        //     }
        // });
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
