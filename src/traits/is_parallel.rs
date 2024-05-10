use crate::traits::eps::Eps;

pub trait IsParallel {
    fn is_parallel(&self, other: &Self, eps: Option<Eps>) -> bool;
}
