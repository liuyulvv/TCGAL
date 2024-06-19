use crate::data_structure::avl_tree::{AVLTree, AVLTreeOption};
use crate::data_structure::priority_queue::PriorityQueue;
use crate::kernel::arc_segment_2::ArcSegment2;
use crate::kernel::line_segment_2::LineSegment2;
use crate::kernel::number_type::NumberType;
use crate::kernel::point_2::Point2;
use crate::kernel::segment_2::{self, Segment2};
use crate::kernel::util_enum::Segment2Type;

#[derive(Debug, Clone, Copy)]
enum StatusNodeSegment<T: NumberType> {
    LineSegment2(LineSegment2<T>),
    ArcSegment2(ArcSegment2<T>),
}

#[derive(Debug, Clone, Copy)]
struct StatusNode<T: NumberType> {
    value: T,
    segment: StatusNodeSegment<T>,
}

pub struct SweepSegment2Intersection<T: NumberType> {
    segments: Vec<StatusNodeSegment<T>>,
    event_queue: PriorityQueue<Point2<T>>,
    status_tree: AVLTree<StatusNode<T>>,
    intersection_points: AVLTree<Point2<T>>,
    last_event_point: Option<Point2<T>>,
}

impl<T: NumberType> SweepSegment2Intersection<T> {
    pub fn new(input_segments: &Vec<impl Segment2<T>>) -> Self {
        let mut segments = Vec::new();
        for segment in input_segments {
            match segment.segment_type() {
                Segment2Type::LineSegment2 => {
                    let source = segment.source();
                    let target = segment.target();
                    if source > target {
                        segments.push(StatusNodeSegment::LineSegment2(LineSegment2::new(
                            source, target,
                        )));
                    } else {
                        segments.push(StatusNodeSegment::LineSegment2(LineSegment2::new(
                            target, source,
                        )));
                    }
                }
                Segment2Type::CircleSegment2 => {
                    todo!()
                }
                Segment2Type::ArcSegment2 => {
                    todo!()
                }
            }
        }
        Self {
            segments,
            event_queue: PriorityQueue::new(),
            status_tree: AVLTree::new(AVLTreeOption::SameNodeInsertRight),
            intersection_points: AVLTree::new(AVLTreeOption::DisableSameNode),
            last_event_point: None,
        }
    }

    pub fn intersection(&mut self) -> Vec<Point2<T>> {
        self.event_queue.clear();
        self.status_tree.clear();
        self.intersection_points.clear();
        for segment in &self.segments {
            match segment {
                StatusNodeSegment::LineSegment2(segment) => {
                    self.event_queue.push(segment.source());
                    self.event_queue.push(segment.target());
                }
                StatusNodeSegment::ArcSegment2(segment) => {
                    self.event_queue.push(segment.source());
                    self.event_queue.push(segment.target());
                }
            }
        }
        self.intersection_points.mid_order_traversal()
    }
}

fn compare_segments<T: NumberType>(
    segment_a: &impl Segment2<T>,
    segment_b: &impl Segment2<T>,
) -> std::cmp::Ordering {
    match segment_a.segment_type() {
        Segment2Type::LineSegment2 => match segment_b.segment_type() {
            Segment2Type::LineSegment2 => {
                let source = segment_a.source();
                let target = segment_a.target();
                let other_source = segment_b.source();
                let other_target = segment_b.target();
                if source.equals(&other_source) && target.equals(&other_target) {
                    return std::cmp::Ordering::Equal;
                } else {
                    if target.y().equals(other_target.y()) {
                        if target.x() < other_target.x() {
                            return std::cmp::Ordering::Greater;
                        } else {
                            return std::cmp::Ordering::Less;
                        }
                    } else if target.y() < other_target.y() {
                        return std::cmp::Ordering::Less;
                    } else {
                        return std::cmp::Ordering::Greater;
                    }
                }
            }
            _ => {
                todo!()
            }
        },
        _ => {
            todo!()
        }
    }
}

impl<T: NumberType> Eq for StatusNode<T> {}

impl<T: NumberType> PartialEq for StatusNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match self.segment {
            StatusNodeSegment::LineSegment2(segment) => match other.segment {
                StatusNodeSegment::LineSegment2(other_segment) => {
                    segment.source().equals(&other_segment.source())
                        && segment.target().equals(&other_segment.target())
                }
                _ => false,
            },
            StatusNodeSegment::ArcSegment2(segment) => match other.segment {
                StatusNodeSegment::ArcSegment2(other_segment) => {
                    segment.center().equals(&other_segment.center())
                        && segment.radius().equals(other_segment.radius())
                        && segment.source().equals(&other_segment.source())
                        && segment.target().equals(&other_segment.target())
                        && segment.orientation() == other_segment.orientation()
                }
                _ => false,
            },
        }
    }
}

impl<T: NumberType> Ord for StatusNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value;
        let other_value = other.value;
        if self.value.equals(other.value) {
            match self.segment {
                StatusNodeSegment::LineSegment2(segment) => match other.segment {
                    StatusNodeSegment::LineSegment2(other_segment) => {
                        let source = segment.source();
                        let target = segment.target();
                        let other_source = other_segment.source();
                        let other_target = other_segment.target();
                        if source.equals(&other_source) && target.equals(&other_target) {
                            return std::cmp::Ordering::Equal;
                        } else {
                            if target.y().equals(other_target.y()) {
                                if target.x() < other_target.x() {
                                    return std::cmp::Ordering::Greater;
                                } else {
                                    return std::cmp::Ordering::Less;
                                }
                            } else if target.y() < other_target.y() {
                                return std::cmp::Ordering::Less;
                            } else {
                                return std::cmp::Ordering::Greater;
                            }
                        }
                    }
                    _ => {
                        panic!("Error: StatusNodeSegment is not LineSegment2")
                    }
                },
                StatusNodeSegment::ArcSegment2(segment) => segment,
            };
        }
        if self_value < other_value {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}

impl<T: NumberType> PartialOrd for StatusNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
