use crate::algorithm::intersection::sweep_segment_2_intersection::SweepSegment2Intersection;

use super::{
    line_segment_2::LineSegment2, number_type::NumberType, point_2::Point2, triangle_2::Triangle2,
    util_enum::TurnDirection,
};

pub struct Polygon2<T: NumberType> {
    vertices: Vec<Point2<T>>,
    edges: Vec<LineSegment2<T>>,
}

impl<T: NumberType> Polygon2<T> {
    pub fn new(vertices: Vec<Point2<T>>) -> Self {
        let edges = vertices
            .iter()
            .zip(vertices.iter().cycle().skip(1))
            .map(|(source, target)| LineSegment2::new(*source, *target))
            .collect();
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> Vec<Point2<T>> {
        self.vertices.clone()
    }

    pub fn edges(&self) -> Vec<LineSegment2<T>> {
        self.edges.clone()
    }

    pub fn area(&self) -> T {
        if self.vertices.len() < 3 {
            return T::zero();
        }
        let p = self.vertices[0];
        let mut area = T::zero();
        for i in 1..self.vertices.len() - 1 {
            let q = self.vertices[(i) % self.vertices.len()];
            let r = self.vertices[(i + 1) % self.vertices.len()];
            let triangle = Triangle2::new(p, q, r);
            area = area + triangle.area();
        }
        return area;
    }

    pub fn is_simple(&self) -> bool {
        let edges = self.edges();
        let mut sweep = SweepSegment2Intersection::new();
        for edge in edges.iter() {
            sweep.push_segment(edge);
        }
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
        assert_eq!(polygon.area(), 100.0);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let p3 = Point2::new(10.0, 0.0);
        let p4 = Point2::new(0.0, 10.0);
        let polygon = Polygon2::new(vec![p1, p2, p3, p4]);
        assert_eq!(polygon.is_simple(), false);
        assert_eq!(polygon.is_convex(), false);
        assert_eq!(polygon.area(), 0.0);

        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(10.0, 10.0);
        let polygon = Polygon2::new(vec![p1, p2]);
        assert_eq!(polygon.is_simple(), true);
        assert_eq!(polygon.is_convex(), true);
    }
}
