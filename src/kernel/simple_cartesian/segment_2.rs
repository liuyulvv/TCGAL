use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

#[derive(Debug, Clone, Copy)]
pub struct Segment2<T: BaseNumberTypeTrait> {
    source: T,
    target: T,
}

impl<T: BaseNumberTypeTrait> Segment2<T> {
    pub fn new(source: T, target: T) -> Self {
        Self { source, target }
    }

    pub fn source(&self) -> T {
        self.source
    }

    pub fn target(&self) -> T {
        self.target
    }
}
