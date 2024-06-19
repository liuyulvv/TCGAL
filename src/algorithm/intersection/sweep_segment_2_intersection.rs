use crate::algorithm::location::point_2_arc_segment_2::{
    is_point_2_on_arc_segment_2, locate_point_2_arc_segment_2,
};
use crate::algorithm::location::point_2_circle_segment_2::is_point_2_on_circle_segment_2;
use crate::algorithm::location::point_2_line_segment_2::is_point_2_on_line_segment_2;
use crate::data_structure::avl_tree::{AVLTree, AVLTreeOption};
use crate::data_structure::priority_queue::PriorityQueue;
use crate::kernel::arc_segment_2::{self, ArcSegment2};
use crate::kernel::circle_segment_2::{self, CircleSegment2};
use crate::kernel::line_segment_2::{self, LineSegment2};
use crate::kernel::number_type::NumberType;
use crate::kernel::point_2::Point2;
use crate::kernel::segment_2::{self, Segment2};
use crate::kernel::util_enum::{Orientation, Segment2Type};
use crate::kernel::vector_2::Vector2;

#[derive(Debug, Clone, Copy)]
enum StatusNodeSegment<T: NumberType> {
    LineSegment2(LineSegment2<T>),
    ArcSegment2(ArcSegment2<T>),
}

#[derive(Debug, Clone, Copy)]
struct StatusNode<T: NumberType> {
    value: T,
    point: Point2<T>,
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
                    let circle_segment = CircleSegment2::new(segment.center(), segment.radius());
                    segments.push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
                        circle_segment.clone(),
                        T::pi(),
                        T::zero(),
                    )));
                    segments.push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
                        circle_segment.clone(),
                        T::pi(),
                        T::pi() * T::from_f64(2.0),
                    )));
                }
                Segment2Type::ArcSegment2 => {
                    let source = segment.source();
                    let target = segment.target();
                    let center = segment.center();
                    let radius = segment.radius();
                    let circle_segment = CircleSegment2::new(center, radius);

                    let source_vector = (source - center).normalize();
                    let target_vector = (target - center).normalize();
                    let vector = Vector2::new(T::from_f64(2.0), T::zero());
                    let source_radian = vector.radian_to(&source_vector);
                    let target_radian = vector.radian_to(&target_vector);

                    let arc_segment =
                        ArcSegment2::new(circle_segment.clone(), source_radian, target_radian);
                    let arc_segments = arc_segment.monotone();
                    for arc_segment in arc_segments {
                        let source = arc_segment.source();
                        let target = arc_segment.target();
                        let source_radian = vector.radian_to(&source_vector);
                        let target_radian = vector.radian_to(&target_vector);
                        if source.x() < target.x() {
                            segments.push(StatusNodeSegment::ArcSegment2(arc_segment));
                        } else {
                            let circle_segment = CircleSegment2::new(center, radius);
                            let arc_segment =
                                ArcSegment2::new(circle_segment, target_radian, source_radian);
                            segments.push(StatusNodeSegment::ArcSegment2(arc_segment));
                        }
                    }
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
        while !self.event_queue.is_empty() {
            let event_point = self.event_queue.pop().unwrap();
            self.handle_event_point(&event_point);
            self.last_event_point = Some(event_point);
        }
        self.intersection_points.mid_order_traversal()
    }

    fn handle_event_point(&mut self, event_point: &Point2<T>) {
        let source_is_p = self.get_segment_with_source(event_point);
        let target_is_p = self.get_active_segment_with_target(event_point);
        let contain_p = self.get_active_segment_containing_point(event_point);
        if source_is_p.is_empty() && target_is_p.is_empty() && contain_p.is_empty() {
            return;
        }
        if source_is_p.len() + target_is_p.len() + contain_p.len() > 1 {
            self.intersection_points.insert(event_point.clone());
        }
        for segment in &target_is_p {
            todo!()
        }
        for segment in &contain_p {
            todo!()
        }
        let source_is_p_empty = source_is_p.is_empty();
        let contain_p_empty = contain_p.is_empty();
        let old_status_nodes = self.status_tree.mid_order_traversal();
        self.status_tree.clear();
        let mut reinserted_segments = Vec::new();
        for status_node in old_status_nodes {
            reinserted_segments.push(status_node.segment.clone());
        }
        for segment in &source_is_p {
            reinserted_segments.push(segment.clone());
        }
        for segment in &contain_p {
            reinserted_segments.push(segment.clone());
        }
        // reinserted_segments.sort_by(compare)
        for segment in reinserted_segments {
            match segment {
                StatusNodeSegment::LineSegment2(line_segment) => {}
                StatusNodeSegment::ArcSegment2(arc_segment) => {}
            }
            // self.status_tree.insert(StatusNode{
            //     value:self.calculate_segment_value(segment, point)
            // })
        }
    }

    fn get_segment_with_source(&self, event_point: &Point2<T>) -> Vec<StatusNodeSegment<T>> {
        let mut result = Vec::new();
        for segment in &self.segments {
            match segment {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    if line_segment.source().equals(event_point) {
                        result.push(segment.clone());
                    }
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    if arc_segment.source().equals(event_point) {
                        result.push(segment.clone());
                    }
                }
            }
        }
        result
    }

    fn get_active_segment_with_target(&self, target: &Point2<T>) -> Vec<StatusNodeSegment<T>> {
        let mut result = Vec::new();
        let status_nodes = self.status_tree.mid_order_traversal();
        for status_node in status_nodes {
            match status_node.segment {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    if line_segment.target().equals(target) {
                        result.push(status_node.segment);
                    }
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    if arc_segment.target().equals(target) {
                        result.push(status_node.segment);
                    }
                }
            }
        }
        result
    }

    fn get_active_segment_containing_point(&self, point: &Point2<T>) -> Vec<StatusNodeSegment<T>> {
        let mut result = Vec::new();
        let status_nodes = self.status_tree.mid_order_traversal();
        for status_node in status_nodes {
            match status_node.segment {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    let source = line_segment.source();
                    let target = line_segment.target();
                    if source.equals(point) || target.equals(point) {
                        continue;
                    }
                    if is_point_2_on_line_segment_2(point, &line_segment) {
                        result.push(status_node.segment);
                    }
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    let source = arc_segment.source();
                    let target = arc_segment.target();
                    if source.equals(point) || target.equals(point) {
                        continue;
                    }
                    if is_point_2_on_arc_segment_2(point, &arc_segment) {
                        result.push(status_node.segment);
                    }
                }
            }
        }
        result
    }

    fn calculate_segment_value(&self, segment: &impl Segment2<T>, point: &Point2<T>) -> T {
        match segment.segment_type() {
            Segment2Type::LineSegment2 => {
                let source = segment.source();
                let target = segment.target();
                if source.x().equals(target.x()) {
                    return point.y();
                }
                let y = source.y()
                    + (point.x() - source.x()) * (target.y() - source.y())
                        / (target.x() - source.x());
                y
            }
            _ => {
                let radius = segment.radius();
                let center = segment.center();
                let circle_segment = CircleSegment2::new(center.clone(), radius.clone());
                let y = radius * radius - (point.x() - center.x()) * (point.x() - center.x());
                let y = y.sqrt();
                let y_a = center.y() + y;
                let y_b = center.y() - y;
                let p_a = Point2::new(point.x(), y_a);
                let p_b = Point2::new(point.x(), y_b);
                if is_point_2_on_arc_segment_2(&p_a, &circle_segment) {
                    y_a
                } else if is_point_2_on_arc_segment_2(&p_b, &circle_segment) {
                    y_b
                } else {
                    point.y()
                }
            }
        }
    }

    fn compare_segments(
        &self,
        segment_a: &impl Segment2<T>,
        segment_b: &impl Segment2<T>,
        event_point: &Point2<T>,
    ) -> std::cmp::Ordering {
        let segment_a_value = self.calculate_segment_value(segment_a, event_point);
        let segment_b_value = self.calculate_segment_value(segment_b, event_point);
        if segment_a_value.equals(segment_b_value) {
            let segment_a_slope = match segment_a.segment_type() {
                Segment2Type::LineSegment2 => {
                    self.calculate_slope(&segment_a.source(), &segment_a.target())
                }
                _ => self.calculate_tangent_slope(&segment_a.center(), event_point),
            };
            let segment_b_slope = match segment_b.segment_type() {
                Segment2Type::LineSegment2 => {
                    self.calculate_slope(&segment_b.source(), &segment_b.target())
                }
                _ => self.calculate_tangent_slope(&segment_b.center(), event_point),
            };
            match segment_a_slope {
                Some(a_slope) => match segment_b_slope {
                    Some(b_slope) => {
                        if a_slope.equals(b_slope) {
                            return std::cmp::Ordering::Equal;
                        } else if a_slope < b_slope {
                            return std::cmp::Ordering::Less;
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    None => {
                        return std::cmp::Ordering::Less;
                    }
                },
                None => match segment_b_slope {
                    Some(_) => {
                        return std::cmp::Ordering::Greater;
                    }
                    None => {
                        if segment_a.segment_type() != Segment2Type::LineSegment2 {
                            return std::cmp::Ordering::Less;
                        } else if segment_b.segment_type() != Segment2Type::LineSegment2 {
                            return std::cmp::Ordering::Greater;
                        } else {
                            return segment_a.target().cmp(&segment_b.target());
                        }
                    }
                },
            }
        } else if segment_a_value < segment_b_value {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }

    fn calculate_slope(&self, source: &Point2<T>, target: &Point2<T>) -> Option<T> {
        let x = target.x() - source.x();
        let y = target.y() - source.y();
        if x.equals(T::zero()) {
            return None;
        }
        Some(y / x)
    }

    fn calculate_tangent_slope(&self, center: &Point2<T>, point: &Point2<T>) -> Option<T> {
        let x = point.x() - center.x();
        let y = point.y() - center.y();
        if y.equals(T::zero()) {
            return None;
        }
        Some(-x / y)
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
