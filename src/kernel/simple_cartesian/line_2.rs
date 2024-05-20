use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

#[derive(Debug, Clone, Copy)]
pub struct Line2<T: BaseNumberTypeTrait> {
    a: T,
    b: T,
    c: T,
}

impl<T: BaseNumberTypeTrait> Line2<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }

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

impl<T: BaseNumberTypeTrait> PartialEq for Line2<T> {
    fn eq(&self, other: &Self) -> bool {
        let det = self.a * other.b - self.b * other.a;
        let det_c = self.c * other.b - self.b * other.c;
        let det_a = self.a * other.c - self.c * other.a;
        let eps = T::default_eps();
        return det.abs() < eps && det_c < eps && det_a < eps;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
