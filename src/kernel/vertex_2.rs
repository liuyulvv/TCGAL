use std::{cell::RefCell, rc::Rc};

use super::{edge_2::Edge2, number_type::NumberType, point_2::Point2};

#[derive(Debug, Clone)]
pub struct Vertex2<T: NumberType> {
    x: T,
    y: T,
    edges: Vec<Rc<RefCell<Edge2<T>>>>,
}

impl<T: NumberType> Vertex2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            edges: Vec::new(),
        }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn edges(&self) -> Vec<Rc<RefCell<Edge2<T>>>> {
        self.edges.clone()
    }

    pub fn add_edge(&mut self, edge: Rc<RefCell<Edge2<T>>>) {
        self.edges.push(edge);
    }

    pub fn remove_edge(&mut self, edge: Rc<RefCell<Edge2<T>>>) {
        self.edges.retain(|e| e != &edge);
    }

    pub fn equals(&self, other: &Self) -> bool {
        let eps = T::default_eps();
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
            < eps
    }

    pub fn to_point(&self) -> Point2<T> {
        Point2::new(self.x, self.y)
    }
}

impl<'a, T: NumberType> PartialEq for Vertex2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<'a, T: NumberType> PartialOrd for Vertex2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.y() > other.y() {
            return Some(std::cmp::Ordering::Greater);
        } else if self.y() < other.y() {
            return Some(std::cmp::Ordering::Less);
        } else {
            if self.x() > other.x() {
                return Some(std::cmp::Ordering::Less);
            } else if self.x() < other.x() {
                return Some(std::cmp::Ordering::Greater);
            } else {
                return Some(std::cmp::Ordering::Equal);
            }
        }
    }
}
