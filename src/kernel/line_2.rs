use super::{number_type::NumberType, point_2::Point2};

/// a*x + b*y + c = 0
#[derive(Debug, Clone, Copy)]
pub struct Line2<T: NumberType> {
    a: T,
    b: T,
    c: T,
}

impl<T: NumberType> Line2<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }

    pub fn from_points(start: &Point2<T>, end: &Point2<T>) -> Self {
        if start.x().equals(end.x()) {
            return Self {
                a: T::from_f64(1.0),
                b: T::zero(),
                c: -start.x(),
            };
        } else if start.y().equals(end.y()) {
            return Self {
                a: T::zero(),
                b: T::from_f64(1.0),
                c: -start.y(),
            };
        } else {
            let a = (end.y() - start.y()) / (end.x() - start.x());
            let b = T::from_f64(-1.0);
            let c = start.y() - a * start.x();
            Self { a, b, c }
        }
    }
}

impl<T: NumberType> Line2<T> {
    pub fn a(&self) -> T {
        self.a
    }

    pub fn b(&self) -> T {
        self.b
    }

    pub fn c(&self) -> T {
        self.c
    }
}

impl<T: NumberType> PartialEq for Line2<T> {
    fn eq(&self, other: &Self) -> bool {
        let det = self.a * other.b - self.b * other.a;
        let det_c = self.c * other.b - self.b * other.c;
        let det_a = self.a * other.c - self.c * other.a;
        let eps = T::default_eps();
        return det.abs() < eps && det_c.abs() < eps && det_a.abs() < eps;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line2() {
        let line1 = Line2::new(1.0, 2.0, 3.0);
        let line2 = Line2::new(2.0, 4.0, 6.0);
        assert_eq!(line1, line2);

        let line3 = Line2::new(1.0, 2.0, 3.0);
        let line4 = Line2::new(2.0, 4.0, 7.0);
        assert_ne!(line3, line4);

        let start = Point2::new(1.0, 2.0);
        let end = Point2::new(3.0, 4.0);
        let line5 = Line2::from_points(&start, &end);
        let line6 = Line2::new(1.0, -1.0, 1.0);
        assert_eq!(line5, line6);

        let start = Point2::new(1.0, 2.0);
        let end = Point2::new(1.0, 4.0);
        let line7 = Line2::from_points(&start, &end);
        let line8 = Line2::new(1.0, 0.0, -1.0);
        assert_eq!(line7, line8);
    }
}
