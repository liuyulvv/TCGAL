use crate::geometry::point_2::Point2;

use super::edge_2::Edge2;

pub struct Vertex2 {
    pub point: Point2,
    pub edge: Option<Box<Edge2>>,
}

impl Vertex2 {
    pub fn new(point: Point2, edge: Option<Box<Edge2>>) -> Self {
        Self { point, edge }
    }
}

impl Default for Vertex2 {
    fn default() -> Self {
        Self {
            point: Point2::default(),
            edge: None,
        }
    }
}
