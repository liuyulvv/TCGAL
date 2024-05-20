use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_edge_2::BaseEdge2, base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

pub trait BaseArrangement2<'a, T: BaseNumberTypeTrait> {
    fn vertices(&self) -> Vec<Box<dyn BaseVertex2<'a, T> + 'a>>;
    fn edges(&self) -> Vec<Box<dyn BaseEdge2<'a, T> + 'a>>;
    fn faces(&self) -> Vec<Box<dyn BaseFace2<'a, T> + 'a>>;
}
