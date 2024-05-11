use super::eps::Eps;

/// This trait provides a method for checking if two objects are the same, within a certain tolerance. The direction is not considered.
pub trait IsSame {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool;
}
