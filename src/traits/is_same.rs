use crate::traits::eps::Eps;

pub trait IsSame {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool;
}
