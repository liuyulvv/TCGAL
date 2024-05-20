use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

#[derive(Debug, Clone, Copy)]
pub struct Point2<T: BaseNumberTypeTrait> {
    x: T,
    y: T,
}

impl<T: BaseNumberTypeTrait> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T: BaseNumberTypeTrait> PartialEq for Point2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
