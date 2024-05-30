use crate::{
    algorithm::{
        arrangement::util::{
            event_queue::EventQueue, event_vertex_2::EventVertex2Type,
            status_structure::StatusStructure,
        },
        intersection::edge_2_edge_2::edge_2_edge_2_intersection,
    },
    kernel::base_dcel::{base_arrangement_2::BaseArrangement2, base_edge_2::BaseEdge2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

#[derive(Debug, Clone)]
pub struct Arrangement2<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> {
    vertices: Vec<T::Vertex>,
    edges: Vec<T>,
    faces: Vec<T::Face>,
}

impl<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> BaseArrangement2<'a, NT>
    for Arrangement2<'a, NT, T>
{
    type Vertex = T::Vertex;
    type Edge = T;
    type Face = T::Face;

    fn new(edges: Vec<T>) -> Self {
        Self {
            vertices: Vec::new(),
            edges,
            faces: Vec::new(),
        }
    }

    fn make_arrangement(&'a mut self) {
        let mut event_queue = EventQueue::new();
        let mut status_structure = StatusStructure::new();

        for edge in self.edges.iter() {
            event_queue.insert_edge(edge);
        }

        while !event_queue.is_empty() {
            let vertex = event_queue.pop();
            match vertex {
                Some(v) => {
                    let vertex_type = v.vertex_type();
                    match vertex_type {
                        EventVertex2Type::Start => {
                            v.edges().iter().for_each(|e| {
                                let neighbors = status_structure.insert(*e);
                                match neighbors {
                                    Some(neighbors) => {
                                        match neighbors.left {
                                            Some(left) => {
                                                let intersection =
                                                    edge_2_edge_2_intersection(left, e);
                                                intersection.iter().for_each(|point| {
                                                    println!("Intersection: {:#?}", point);
                                                });
                                            }
                                            None => {}
                                        }
                                        match neighbors.right {
                                            Some(right) => {
                                                println!("right: {:#?}", right);
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            });
                        }
                        EventVertex2Type::End => {
                            v.edges().iter().for_each(|e| {
                                status_structure.remove(*e);
                            });
                        }
                        EventVertex2Type::Intersection => {
                            println!("Intersection");
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    fn vertices(&self) -> Vec<&Self::Vertex> {
        self.vertices.iter().collect()
    }

    fn edges(&self) -> Vec<&Self::Edge> {
        self.edges.iter().collect()
    }

    fn faces(&self) -> Vec<&Self::Face> {
        self.faces.iter().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::kernel::{
        base_dcel::{base_edge_2::BaseEdge2, base_vertex_2::BaseVertex2},
        simple_cartesian::{edge_2::Edge2, vertex_2::Vertex2},
    };

    #[test]
    fn test_arrangement_2() {
        let mut edges = Vec::new();

        let source = Vertex2::new(10.0, 0.0);
        let target = Vertex2::new(0.0, 10.0);
        let edge = Edge2::new_segment(&source, &target);
        edges.push(edge);

        let source = Vertex2::new(0.0, 0.0);
        let target = Vertex2::new(10.0, 10.0);
        let edge = Edge2::new_segment(&source, &target);
        edges.push(edge);

        let mut arrangement = Arrangement2::new(edges);
        arrangement.make_arrangement();
    }
}
