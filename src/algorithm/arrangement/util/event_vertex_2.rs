use crate::{
    kernel::base_dcel::{base_edge_2::BaseEdge2, base_vertex_2::BaseVertex2},
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventVertex2Type {
    Start,
    End,
    Intersection,
}

#[derive(Debug)]
pub struct EventVertex2<'a, NT: BaseNumberTypeTrait, T>
where
    T: BaseEdge2<'a, NT>,
    T::Vertex: BaseVertex2<'a, NT>,
{
    vertex: &'a T::Vertex,
    vertex_type: EventVertex2Type,
    edges: Vec<&'a T>,
}

impl<'a, NT: BaseNumberTypeTrait, T> EventVertex2<'a, NT, T>
where
    T: BaseEdge2<'a, NT>,
    T::Vertex: BaseVertex2<'a, NT>,
{
    pub fn new(vertex: &'a T::Vertex, vertex_type: EventVertex2Type) -> Self {
        Self {
            vertex,
            vertex_type,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: &'a T) {
        self.edges.push(edge);
    }

    pub fn vertex(&self) -> &'a T::Vertex {
        self.vertex
    }

    pub fn vertex_type(&self) -> EventVertex2Type {
        self.vertex_type.clone()
    }

    pub fn edges(&self) -> &Vec<&'a T> {
        &self.edges
    }
}

impl<'a, NT: BaseNumberTypeTrait, T> PartialEq for EventVertex2<'a, NT, T>
where
    T: BaseEdge2<'a, NT>,
    T::Vertex: BaseVertex2<'a, NT>,
{
    fn eq(&self, other: &Self) -> bool {
        self.vertex.equals(other.vertex)
    }
}

impl<'a, Nt: BaseNumberTypeTrait, T> Eq for EventVertex2<'a, Nt, T>
where
    T: BaseEdge2<'a, Nt>,
    T::Vertex: BaseVertex2<'a, Nt>,
{
}

impl<'a, NT: BaseNumberTypeTrait, T> Ord for EventVertex2<'a, NT, T>
where
    T: BaseEdge2<'a, NT>,
    T::Vertex: BaseVertex2<'a, NT>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.vertex.equals(other.vertex) {
            return std::cmp::Ordering::Equal;
        }
        if self.vertex.y() > other.vertex.y() {
            std::cmp::Ordering::Greater
        } else if self.vertex.y() < other.vertex.y() {
            std::cmp::Ordering::Less
        } else {
            if self.vertex.x() > other.vertex.x() {
                std::cmp::Ordering::Greater
            } else if self.vertex.x() < other.vertex.x() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl<'a, NT: BaseNumberTypeTrait, T> PartialOrd for EventVertex2<'a, NT, T>
where
    T: BaseEdge2<'a, NT>,
    T::Vertex: BaseVertex2<'a, NT>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.vertex.equals(other.vertex) {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.vertex.y() > other.vertex.y() {
            Some(std::cmp::Ordering::Greater)
        } else if self.vertex.y() < other.vertex.y() {
            Some(std::cmp::Ordering::Less)
        } else {
            if self.vertex.x() > other.vertex.x() {
                Some(std::cmp::Ordering::Greater)
            } else if self.vertex.x() < other.vertex.x() {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}
