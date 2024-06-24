use std::fmt::Debug;

use super::{number_type::NumberType, point_2::Point2, util_enum::Segment2Type};

/** Segment2 trait
 *
 * - LineSegment2 has implemented this trait except for the **source_radian**, **target_radian**, **center** and **radius**, methods.
 *
 * - CircleSegment2 has implemented this trait except for the **source**, **source_radian**, **target** and **target_radian** methods.
 *
 * - ArcSegment2 has implemented this trait.
 */

pub trait Segment2<T: NumberType>: Debug + Clone + Copy {
    fn segment_type(&self) -> Segment2Type;
    fn source(&self) -> Point2<T>;
    fn source_radian(&self) -> T;
    fn target(&self) -> Point2<T>;
    fn target_radian(&self) -> T;
    fn center(&self) -> Point2<T>;
    fn radius(&self) -> T;
}
