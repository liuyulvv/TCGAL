use crate::number_type::base_number_type_trait::BaseNumberTypeTrait;

use super::{base_face_2::BaseFace2, base_vertex_2::BaseVertex2};

pub trait BaseEdge2<'a, T: BaseNumberTypeTrait> {
    fn source(&self) -> Box<dyn BaseVertex2<'a, T> + 'a>;
    fn target(&self) -> Box<dyn BaseVertex2<'a, T> + 'a>;
    fn twin(&self) -> Option<Box<dyn BaseEdge2<'a, T> + 'a>>;
    fn next(&self) -> Option<Box<dyn BaseEdge2<'a, T> + 'a>>;
    fn prev(&self) -> Option<Box<dyn BaseEdge2<'a, T> + 'a>>;
    fn face(&self) -> Option<Box<dyn BaseFace2<'a, T> + 'a>>;
}
