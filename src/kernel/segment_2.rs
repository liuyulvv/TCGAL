use std::fmt::Debug;

use super::{
    number_type::NumberType,
    point_2::Point2,
    util_enum::{Orientation, Segment2Type},
};

/** Segment2 trait
 *
 * - LineSegment2 has implemented this trait except for the **center**, **radius**, **orientation**, and **reverse_orientation** methods.
 *
 * - CircleSegment2 has implemented this trait except for the **source**, **target**, **orientation** and **reverse_orientation** methods.
 *
 * - ArcSegment2 has implemented this trait.
 */

pub trait Segment2<T: NumberType>: Debug + Clone + Copy {
    fn segment_type(&self) -> Segment2Type;
    fn source(&self) -> Point2<T>;
    fn target(&self) -> Point2<T>;
    fn center(&self) -> Point2<T>;
    fn radius(&self) -> T;
    fn orientation(&self) -> Orientation;
    fn reverse_orientation(&mut self);
}
