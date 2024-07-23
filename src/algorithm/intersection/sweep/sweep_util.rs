use crate::{
    algorithm::location::point_2_arc_segment_2::is_point_2_on_arc_segment_2,
    kernel::{
        arc_segment_2::ArcSegment2, circle_segment_2::CircleSegment2, number_type::NumberType,
        point_2::Point2, segment_2::Segment2, util_enum::Segment2Type,
    },
};

pub fn compare_segments<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
    event_point: &Point2<T>,
) -> std::cmp::Ordering {
    let segment_a_value = calculate_segment_value(segment_a, event_point);
    let segment_b_value = calculate_segment_value(segment_b, event_point);
    if segment_a_value.equals(segment_b_value) {
        let segment_a_slope = match segment_a.segment_type() {
            Segment2Type::LineSegment2 => calculate_slope(&segment_a.source(), &segment_a.target()),
            _ => calculate_tangent_slope(&segment_a.center(), event_point),
        };
        let segment_b_slope = match segment_b.segment_type() {
            Segment2Type::LineSegment2 => calculate_slope(&segment_b.source(), &segment_b.target()),
            _ => calculate_tangent_slope(&segment_b.center(), event_point),
        };
        match segment_a_slope {
            Some(a_slope) => match segment_b_slope {
                Some(b_slope) => {
                    if a_slope.equals(b_slope) {
                        compare_segments_same_slope(segment_a, segment_b, event_point)
                    } else if a_slope < b_slope {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                None => std::cmp::Ordering::Less,
            },
            None => match segment_b_slope {
                Some(_) => std::cmp::Ordering::Greater,
                None => compare_segments_same_slope(segment_a, segment_b, event_point),
            },
        }
    } else if segment_a_value < segment_b_value {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

pub fn calculate_segment_value<T: NumberType>(segment: &impl Segment2<T>, point: &Point2<T>) -> T {
    match segment.segment_type() {
        Segment2Type::LineSegment2 => {
            let source = segment.source();
            let target = segment.target();
            if source.x().equals(target.x()) {
                return point.y();
            }
            let y = source.y()
                + (point.x() - source.x()) * (target.y() - source.y()) / (target.x() - source.x());
            y
        }
        _ => {
            let radius = segment.radius();
            let center = segment.center();
            let y = radius * radius - (point.x() - center.x()) * (point.x() - center.x());
            let y = y.sqrt();
            let y_a = center.y() + y;
            let y_b = center.y() - y;
            let p_a = Point2::new(point.x(), y_a);
            let p_b = Point2::new(point.x(), y_b);
            if is_point_2_on_arc_segment_2(&p_a, segment) {
                y_a
            } else if is_point_2_on_arc_segment_2(&p_b, segment) {
                y_b
            } else {
                point.y()
            }
        }
    }
}

fn calculate_slope<T: NumberType>(source: &Point2<T>, target: &Point2<T>) -> Option<T> {
    let x = target.x() - source.x();
    let y = target.y() - source.y();
    if x.equals(T::zero()) {
        return None;
    }
    Some(y / x)
}

fn calculate_tangent_slope<T: NumberType>(center: &Point2<T>, point: &Point2<T>) -> Option<T> {
    let x = point.x() - center.x();
    let y = point.y() - center.y();
    if y.equals(T::zero()) {
        return None;
    }
    Some(-x / y)
}

fn compare_segments_same_slope<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
    event_point: &Point2<T>,
) -> std::cmp::Ordering {
    let (mid_a_value, mid_b_value) = calculate_mid_value(segment_a, segment_b, event_point);
    if mid_a_value.equals(mid_b_value) {
        return std::cmp::Ordering::Equal;
    } else if mid_a_value < mid_b_value {
        return std::cmp::Ordering::Less;
    } else {
        return std::cmp::Ordering::Greater;
    }
}

fn calculate_mid_value<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
    event_point: &Point2<T>,
) -> (T, T) {
    let target_a = get_target_of_segment(segment_a);
    let target_b = get_target_of_segment(segment_b);
    let target = if target_a < target_b {
        target_a
    } else {
        target_b
    };
    let mid = (*event_point + target) * T::from_f64(0.5);
    let mid = Point2::new(mid.x(), mid.y());
    (
        calculate_segment_value(segment_a, &mid),
        calculate_segment_value(segment_b, &mid),
    )
}

fn get_target_of_segment<T: NumberType>(segment: &impl Segment2<T>) -> Point2<T> {
    match segment.segment_type() {
        Segment2Type::LineSegment2 => segment.target(),
        _ => {
            let arc_segment = ArcSegment2::new(
                CircleSegment2::new(segment.center(), segment.radius()),
                segment.source_radian(),
                segment.target_radian(),
            );
            if arc_segment.is_top() {
                arc_segment.source()
            } else {
                arc_segment.target()
            }
        }
    }
}
