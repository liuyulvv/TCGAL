use super::{
    number_type::NumberType, point_2::Point2, segment_2::Segment2, util_enum::Orientation,
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

    pub fn edges(&self) -> [Segment2<T>; 3] {
        [
            Segment2::new(self.a.clone(), self.b.clone()),
            Segment2::new(self.b.clone(), self.c.clone()),
            Segment2::new(self.c.clone(), self.a.clone()),
        ]
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn calculate_orientation(a: &Point2<T>, b: &Point2<T>, c: &Point2<T>) -> Orientation {
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
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_triangle2_calculate_orientation() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        assert_eq!(
            Triangle2::calculate_orientation(&a, &b, &c),
            Orientation::CounterClockwise
        );
        assert_eq!(
            Triangle2::calculate_orientation(&a, &c, &b),
            Orientation::Clockwise
        );
    }

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
                Segment2::new(a.clone(), b.clone()),
                Segment2::new(b.clone(), c.clone()),
                Segment2::new(c.clone(), a.clone())
            ]
        );

        let triangle = Triangle2::new(a.clone(), c.clone(), b.clone());
        assert_eq!(
            triangle.edges(),
            [
                Segment2::new(a.clone(), c.clone()),
                Segment2::new(c.clone(), b.clone()),
                Segment2::new(b.clone(), a.clone())
            ]
        );
    }
}
