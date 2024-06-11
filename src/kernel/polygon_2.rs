use super::{number_type::NumberType, point_2::Point2, segment_2::Segment2};

pub struct Polygon2<T: NumberType> {
    vertices: Vec<Point2<T>>,
    edges: Vec<Segment2<T>>,
}

impl<T: NumberType> Polygon2<T> {
    pub fn new(vertices: Vec<Point2<T>>) -> Self {
        let edges = vertices
            .iter()
            .zip(vertices.iter().cycle().skip(1))
            .map(|(source, target)| Segment2::new(*source, *target))
            .collect();
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> &Vec<Point2<T>> {
        &self.vertices
    }

    pub fn edges(&self) -> &Vec<Segment2<T>> {
        &self.edges
    }

    pub fn area(&self) -> T {
        todo!()
    }

    pub fn is_simple(&self) -> bool {
        todo!()
    }

    pub fn is_convex(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_polygon() {
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 0.0);
        let p3 = Point2::new(10.0, 10.0);
        let p4 = Point2::new(0.0, 10.0);
        let polygon = Polygon2::new(vec![p1, p2, p3, p4]);
        assert_eq!(polygon.vertices().len(), 4);
        assert_eq!(polygon.edges().len(), 4);
    }
}
