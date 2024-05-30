use std::marker::PhantomData;

use crate::{
    kernel::base_dcel::base_edge_2::BaseEdge2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

pub struct Neighbors<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> {
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    phantom: PhantomData<NT>,
}

pub struct StatusStructure<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> {
    vertex_vec: Vec<&'a T::Vertex>,
    edge_vec: Vec<&'a T>,
}

impl<'a, NT: BaseNumberTypeTrait, T: BaseEdge2<'a, NT>> StatusStructure<'a, NT, T> {
    pub fn new() -> Self {
        Self {
            vertex_vec: Vec::new(),
            edge_vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, edge: &'a T) -> Option<Neighbors<'a, NT, T>> {
        let insert_index = self.insert_index(edge);
        match insert_index {
            Some(index) => {
                self.vertex_vec.insert(index, edge.source());
                self.edge_vec.insert(index, edge);
                let left = self.edge_vec.get(index.wrapping_sub(1));
                let right = self.edge_vec.get(index.wrapping_add(1));
                match (left, right) {
                    (Some(left), Some(right)) => Some(Neighbors {
                        left: Some(left),
                        right: Some(right),
                        phantom: PhantomData,
                    }),
                    (Some(left), None) => Some(Neighbors {
                        left: Some(left),
                        right: None,
                        phantom: PhantomData,
                    }),
                    (None, Some(right)) => Some(Neighbors::<NT, T> {
                        left: None,
                        right: Some(right),
                        phantom: PhantomData,
                    }),
                    (None, None) => None,
                }
            }
            None => {
                let is_empty = self.vertex_vec.is_empty();
                self.vertex_vec.push(edge.source());
                self.edge_vec.push(edge);
                if is_empty {
                    None
                } else {
                    let left = self.edge_vec.get(self.edge_vec.len().wrapping_sub(1));
                    Some(Neighbors {
                        left: Some(left.unwrap()),
                        right: None,
                        phantom: PhantomData,
                    })
                }
            }
        }
    }

    pub fn remove(&mut self, edge: &'a T) -> Option<Neighbors<'a, NT, T>> {
        let remove_index = self.edge_vec.iter().position(|&e| e == edge);
        match remove_index {
            Some(index) => {
                self.vertex_vec.remove(index);
                self.edge_vec.remove(index);
                let left = self.edge_vec.get(index.wrapping_sub(1));
                let right = self.edge_vec.get(index);
                match (left, right) {
                    (Some(left), Some(right)) => Some(Neighbors {
                        left: Some(left),
                        right: Some(right),
                        phantom: PhantomData,
                    }),
                    (Some(left), None) => Some(Neighbors {
                        left: Some(left),
                        right: None,
                        phantom: PhantomData,
                    }),
                    (None, Some(right)) => Some(Neighbors::<NT, T> {
                        left: None,
                        right: Some(right),
                        phantom: PhantomData,
                    }),
                    (None, None) => None,
                }
            }
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertex_vec.is_empty() && self.edge_vec.is_empty()
    }

    pub fn insert_index(&self, edge: &'a T) -> Option<usize> {
        let source = edge.source();
        self.vertex_vec.iter().position(|&e| e <= source)
    }

    pub fn vertex_iter(&self) -> impl Iterator<Item = &&'a T::Vertex> {
        self.vertex_vec.iter()
    }

    pub fn edge_iter(&self) -> impl Iterator<Item = &&'a T> {
        self.edge_vec.iter()
    }
}
