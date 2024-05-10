use crate::traits::eps::Eps;

pub trait IsEqual {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool;
}
