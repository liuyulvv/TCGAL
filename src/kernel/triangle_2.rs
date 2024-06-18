use super::{
    line_segment_2::LineSegment2, number_type::NumberType, point_2::Point2, util_enum::Orientation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle2<T: NumberType> {
    a: Point2<T>,
    b: Point2<T>,
    c: Point2<T>,
    orientation: Orientation,
}

impl<T: NumberType> Triangle2<T> {
    pub fn new(a: Point2<T>, b: Point2<T>, c: Point2<T>) -> Self {
        let orientation = Self::calculate_orientation(&a, &b, &c);
        Self {
            a,
            b,
            c,
            orientation,
        }
    }

    pub fn a(&self) -> Point2<T> {
        self.a.clone()
    }

    pub fn b(&self) -> Point2<T> {
        self.b.clone()
    }

    pub fn c(&self) -> Point2<T> {
        self.c.clone()
    }

    pub fn vertices(&self) -> [Point2<T>; 3] {
        [self.a.clone(), self.b.clone(), self.c.clone()]
    }

    pub fn edges(&self) -> [LineSegment2<T>; 3] {
        [
            LineSegment2::new(self.a.clone(), self.b.clone()),
            LineSegment2::new(self.b.clone(), self.c.clone()),
            LineSegment2::new(self.c.clone(), self.a.clone()),
        ]
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn calculate_orientation(a: &Point2<T>, b: &Point2<T>, c: &Point2<T>) -> Orientation {
        let cross_product = (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x());
        if cross_product > T::zero() {
            Orientation::CounterClockwise
        } else {
            Orientation::Clockwise
        }
    }

    pub fn reverse_orientation(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Clockwise => Orientation::CounterClockwise,
            Orientation::CounterClockwise => Orientation::Clockwise,
        };
    }

    pub fn area(&self) -> T {
        let a = self.a();
        let b = self.b();
        let c = self.c();
        let cross = (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x());
        let area = cross.abs() / T::from_f64(2.0);
        match self.orientation {
            Orientation::Clockwise => -area,
            Orientation::CounterClockwise => area,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_triangle2_reverse_orientation() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        let mut triangle = Triangle2::new(a, b, c);
        assert_eq!(triangle.orientation(), Orientation::CounterClockwise);
        triangle.reverse_orientation();
        assert_eq!(triangle.orientation(), Orientation::Clockwise);
    }

    #[test]
    fn test_triangle2_vertices() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        let triangle = Triangle2::new(a.clone(), b.clone(), c.clone());
        assert_eq!(triangle.vertices(), [a, b, c]);
    }

    #[test]
    fn test_triangle2_edges() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        let triangle = Triangle2::new(a.clone(), b.clone(), c.clone());
        assert_eq!(
            triangle.edges(),
            [
                LineSegment2::new(a.clone(), b.clone()),
                LineSegment2::new(b.clone(), c.clone()),
                LineSegment2::new(c.clone(), a.clone())
            ]
        );

        let triangle = Triangle2::new(a.clone(), c.clone(), b.clone());
        assert_eq!(
            triangle.edges(),
            [
                LineSegment2::new(a.clone(), c.clone()),
                LineSegment2::new(c.clone(), b.clone()),
                LineSegment2::new(b.clone(), a.clone())
            ]
        );
    }
}
