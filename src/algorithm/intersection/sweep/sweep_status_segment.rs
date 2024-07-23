use crate::kernel::{
    arc_segment_2::ArcSegment2, line_segment_2::LineSegment2, number_type::NumberType,
};

#[derive(Debug, Clone, Copy)]
pub enum StatusNodeSegment<T: NumberType> {
    LineSegment2(LineSegment2<T>),
    ArcSegment2(ArcSegment2<T>),
}
