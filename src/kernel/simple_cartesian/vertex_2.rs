use crate::{
    kernel::base_dcel::base_vertex_2::BaseVertex2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::edge_2::Edge2;

#[derive(Debug, Clone)]
pub struct Vertex2<'a, NT: BaseNumberTypeTrait> {
    x: NT,
    y: NT,
    edges: Vec<&'a Edge2<'a, NT>>,
}

impl<'a, NT: BaseNumberTypeTrait> BaseVertex2<'a, NT> for Vertex2<'a, NT> {
    type Edge = Edge2<'a, NT>;

    fn new(x: NT, y: NT) -> Self {
        Self {
            x,
            y,
            edges: Vec::new(),
        }
    }

    fn x(&self) -> NT {
        self.x
    }

    fn y(&self) -> NT {
        self.y
    }

    fn edges(&self) -> &Vec<&Self::Edge> {
        &self.edges
    }

    fn add_edge(&mut self, edge: &'a Self::Edge) {
        self.edges.push(edge);
    }

    fn remove_edge(&mut self, edge: &'a Self::Edge) {
        self.edges.retain(|e| e != &edge);
    }

    fn equals(&self, other: &Self) -> bool {
        let eps = NT::default_eps();
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
            < eps
    }
}

impl<'a, NT: BaseNumberTypeTrait> PartialEq for Vertex2<'a, NT> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl<'a, NT: BaseNumberTypeTrait> PartialOrd for Vertex2<'a, NT> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y() > other.y() {
            return Some(std::cmp::Ordering::Greater);
        } else if self.y() < other.y() {
            return Some(std::cmp::Ordering::Less);
        } else {
            if self.x() > other.x() {
                return Some(std::cmp::Ordering::Greater);
            } else if self.x() < other.x() {
                return Some(std::cmp::Ordering::Less);
            } else {
                return Some(std::cmp::Ordering::Equal);
            }
        }
    }
}
