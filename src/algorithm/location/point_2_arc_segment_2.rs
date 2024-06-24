use crate::kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2};

use super::location_enum::Point2ArcSegment2Location;

pub fn is_point_2_on_arc_segment_2<T: NumberType>(
    point: &Point2<T>,
    arc_segment: &impl Segment2<T>,
) -> bool {
    let location = locate_point_2_arc_segment_2(point, arc_segment);
    match location {
        Point2ArcSegment2Location::On => true,
        _ => false,
    }
}

pub fn locate_point_2_arc_segment_2<T: NumberType>(
    point: &Point2<T>,
    arc_segment: &impl Segment2<T>,
) -> Point2ArcSegment2Location {
    let center = arc_segment.center();
    let radius = arc_segment.radius();
    let distance = center.distance(point);
    if distance.equals(radius) {
        let vector = *point - center;
        let source = arc_segment.source() - center;
        let target = arc_segment.target() - center;
        let radian = source.radian_to(&target);
        let radian_source = vector.radian_to(&source);
        let radian_target = vector.radian_to(&target);
        let source_radian = source.radian_to(&vector);
        let target_radian = target.radian_to(&vector);
        if radian_source.equals(T::zero())
            || radian_target.equals(T::zero())
            || source_radian.equals(T::zero())
            || target_radian.equals(T::zero())
        {
            Point2ArcSegment2Location::On
        } else if (source_radian + radian_target).equals(radian) {
            Point2ArcSegment2Location::On
        } else {
            Point2ArcSegment2Location::NotOn
        }
    } else {
        Point2ArcSegment2Location::NotOn
    }
}
