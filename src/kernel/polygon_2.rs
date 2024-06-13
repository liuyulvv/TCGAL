use crate::algorithm::intersection::sweep_line_segment_2_intersection::SweepLineSegment2Intersection;

use super::{
    number_type::NumberType, point_2::Point2, segment_2::Segment2, util_enum::TurnDirection,
};

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

    pub fn vertices(&self) -> Vec<Point2<T>> {
        self.vertices.clone()
    }

    pub fn edges(&self) -> Vec<Segment2<T>> {
        self.edges.clone()
    }

    pub fn area(&self) -> T {
        todo!()
    }

    pub fn is_simple(&self) -> bool {
        let edges = self.edges();
        let mut sweep = SweepLineSegment2Intersection::new(&edges);
        let intersections = sweep.intersection();
        let vertices_size = self.vertices.len();
        if vertices_size == intersections.len() {
            return true;
        }
        return false;
    }

    pub fn is_convex(&self) -> bool {
        let n = self.vertices.len();
        let mut prev_turn: Option<TurnDirection> = None;
        for i in 0..n {
            let p = self.vertices[i];
            let q = self.vertices[(i + 1) % n];
            let r = self.vertices[(i + 2) % n];
            let turn = Point2::turn(&p, &q, &r);
            match prev_turn {
                None => match turn {
                    TurnDirection::Collinear => continue,
                    _ => prev_turn = Some(turn),
                },
                Some(prev) => match turn {
                    TurnDirection::Collinear => continue,
                    _ => {
                        if prev != turn {
                            return false;
                        }
                    }
                },
            }
        }
        true
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
        assert_eq!(polygon.is_simple(), true);
        assert_eq!(polygon.is_convex(), true);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(10.0, 0.0);
        let p4 = Point2::new(0.0, 10.0);
        let polygon = Polygon2::new(vec![p1, p2, p3, p4]);
        assert_eq!(polygon.is_simple(), false);
        assert_eq!(polygon.is_convex(), false);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let polygon = Polygon2::new(vec![p1, p2]);
        assert_eq!(polygon.is_simple(), true);
        assert_eq!(polygon.is_convex(), true);
    }
}
