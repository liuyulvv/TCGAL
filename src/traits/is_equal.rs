use super::eps::Eps;

/// This trait provides a method for checking if two objects are equal, within a certain tolerance. The direction is considered.
pub trait IsEqual {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool;
}
