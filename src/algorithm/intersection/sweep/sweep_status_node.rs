use super::{sweep_status_segment::StatusNodeSegment, sweep_util::compare_segments};
use crate::kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2};
use std::{cell::RefCell, rc::Rc};

type OptionStatusNodeRc<T> = Option<Rc<RefCell<SweepStatusNode<T>>>>;

#[derive(Debug, Clone)]
pub struct SweepStatusNode<T: NumberType> {
    pub value: T,
    pub point: Point2<T>,
    pub segment: Rc<RefCell<StatusNodeSegment<T>>>,
    pub left: OptionStatusNodeRc<T>,
    pub right: OptionStatusNodeRc<T>,
}

impl<T: NumberType> Eq for SweepStatusNode<T> {}

impl<T: NumberType> PartialEq for SweepStatusNode<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.segment, &other.segment)
    }
}

impl<T: NumberType> Ord for SweepStatusNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value;
        let other_value = other.value;
        if self.value.equals(other.value) {
            let mut point = self.point.clone();
            if point > other.point {
                point = other.point.clone();
            }
            let self_segment = self.segment.borrow();
            let other_segment = other.segment.borrow();
            match *self_segment {
                StatusNodeSegment::LineSegment2(segment) => match *other_segment {
                    StatusNodeSegment::LineSegment2(other_segment) => {
                        if segment.source().equals(&other_segment.source())
                            && segment.target().equals(&other_segment.target())
                        {
                            std::cmp::Ordering::Equal
                        } else {
                            compare_segments(&segment, &other_segment, &point)
                        }
                    }
                    StatusNodeSegment::ArcSegment2(other_segment) => {
                        compare_segments(&segment, &other_segment, &point)
                    }
                },
                StatusNodeSegment::ArcSegment2(segment) => match *other_segment {
                    StatusNodeSegment::LineSegment2(other_segment) => {
                        compare_segments(&segment, &other_segment, &point)
                    }
                    StatusNodeSegment::ArcSegment2(other_segment) => {
                        if segment.center().equals(&other_segment.center())
                            && segment.radius().equals(other_segment.radius())
                            && segment.source().equals(&other_segment.source())
                            && segment.target().equals(&other_segment.target())
                        {
                            std::cmp::Ordering::Equal
                        } else {
                            compare_segments(&segment, &other_segment, &point)
                        }
                    }
                },
            }
        } else if self_value < other_value {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl<T: NumberType> PartialOrd for SweepStatusNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
