use crate::kernel::{number_type::NumberType, segment_2::Segment2, util_enum::Segment2Type};

pub fn is_same_segment_2<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
) -> bool {
    match segment_a.segment_type() {
        Segment2Type::LineSegment2 => match segment_b.segment_type() {
            Segment2Type::LineSegment2 => {
                let source_a = segment_a.source();
                let target_a = segment_a.target();
                let source_b = segment_b.source();
                let target_b = segment_b.target();
                source_a == source_b && target_a == target_b
            }
            Segment2Type::ArcSegment2 => false,
            Segment2Type::CircleSegment2 => false,
        },
        Segment2Type::ArcSegment2 => match segment_b.segment_type() {
            Segment2Type::LineSegment2 => false,
            Segment2Type::ArcSegment2 => {
                let center_a = segment_a.center();
                let radius_a = segment_a.radius();
                let center_b = segment_b.center();
                let radius_b = segment_b.radius();
                let source_radian_a = segment_a.source_radian();
                let target_radian_a = segment_a.target_radian();
                let source_radian_b = segment_b.source_radian();
                let target_radian_b = segment_b.target_radian();
                center_a == center_b
                    && radius_a.equals(radius_b)
                    && source_radian_a.equals(source_radian_b)
                    && target_radian_a.equals(target_radian_b)
            }
            Segment2Type::CircleSegment2 => false,
        },
        Segment2Type::CircleSegment2 => match segment_b.segment_type() {
            Segment2Type::LineSegment2 => false,
            Segment2Type::ArcSegment2 => false,
            Segment2Type::CircleSegment2 => {
                let center_a = segment_a.center();
                let radius_a = segment_a.radius();
                let center_b = segment_b.center();
                let radius_b = segment_b.radius();
                center_a == center_b && radius_a.equals(radius_b)
            }
        },
    }
}
