use crate::algorithm::location::point_2_arc_segment_2::is_point_2_on_arc_segment_2;
use crate::algorithm::location::point_2_line_segment_2::is_point_2_on_line_segment_2;
use crate::data_structure::avl_tree::{AVLTree, AVLTreeOption};
use crate::data_structure::priority_queue::PriorityQueue;
use crate::kernel::arc_segment_2::ArcSegment2;
use crate::kernel::circle_segment_2::CircleSegment2;
use crate::kernel::line_segment_2::LineSegment2;
use crate::kernel::number_type::NumberType;
use crate::kernel::point_2::Point2;
use crate::kernel::segment_2::Segment2;
use crate::kernel::util_enum::Segment2Type;
use crate::kernel::vector_2::Vector2;

use super::segment_2_segment_2::segment_2_segment_2_intersection;

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
    pub fn push_segment(&mut self, segment: &impl Segment2<T>) {
        match segment.segment_type() {
            Segment2Type::LineSegment2 => {
                let source = segment.source();
                let target = segment.target();
                if source > target {
                    self.segments
                        .push(StatusNodeSegment::LineSegment2(LineSegment2::new(
                            source, target,
                        )));
                } else {
                    self.segments
                        .push(StatusNodeSegment::LineSegment2(LineSegment2::new(
                            target, source,
                        )));
                }
            }
            Segment2Type::CircleSegment2 => {
                let circle_segment = CircleSegment2::new(segment.center(), segment.radius());
                self.segments
                    .push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
                        circle_segment.clone(),
                        T::pi(),
                        T::zero(),
                    )));
                self.segments
                    .push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
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
                        self.segments
                            .push(StatusNodeSegment::ArcSegment2(arc_segment));
                    } else {
                        let circle_segment = CircleSegment2::new(center, radius);
                        let arc_segment =
                            ArcSegment2::new(circle_segment, target_radian, source_radian);
                        self.segments
                            .push(StatusNodeSegment::ArcSegment2(arc_segment));
                    }
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
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
        let mut event_points = AVLTree::new(AVLTreeOption::DisableSameNode);
        for segment in &self.segments {
            match segment {
                StatusNodeSegment::LineSegment2(segment) => {
                    event_points.insert(segment.source());
                    event_points.insert(segment.target());
                }
                StatusNodeSegment::ArcSegment2(segment) => {
                    event_points.insert(segment.source());
                    event_points.insert(segment.target());
                }
            }
        }
        let event_points = event_points.mid_order_traversal();
        for event_point in event_points {
            self.event_queue.push(event_point);
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
            self.status_tree.delete(StatusNode {
                value: match self.last_event_point {
                    Some(point) => match segment {
                        StatusNodeSegment::LineSegment2(line_segment) => {
                            calculate_segment_value(line_segment, &point)
                        }
                        StatusNodeSegment::ArcSegment2(arc_segment) => {
                            calculate_segment_value(arc_segment, &point)
                        }
                    },
                    None => match segment {
                        StatusNodeSegment::LineSegment2(line_segment) => line_segment.source().y(),
                        StatusNodeSegment::ArcSegment2(arc_segment) => arc_segment.source().y(),
                    },
                },
                point: event_point.clone(),
                segment: segment.clone(),
            })
        }
        for segment in &contain_p {
            self.status_tree.delete(StatusNode {
                value: match self.last_event_point {
                    Some(point) => match segment {
                        StatusNodeSegment::LineSegment2(line_segment) => {
                            calculate_segment_value(line_segment, &point)
                        }
                        StatusNodeSegment::ArcSegment2(arc_segment) => {
                            calculate_segment_value(arc_segment, &point)
                        }
                    },
                    None => match segment {
                        StatusNodeSegment::LineSegment2(line_segment) => line_segment.source().y(),
                        StatusNodeSegment::ArcSegment2(arc_segment) => arc_segment.source().y(),
                    },
                },
                point: event_point.clone(),
                segment: segment.clone(),
            })
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
        reinserted_segments.sort_by(|a, b| match a {
            StatusNodeSegment::LineSegment2(segment) => match b {
                StatusNodeSegment::LineSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
                StatusNodeSegment::ArcSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
            },
            StatusNodeSegment::ArcSegment2(segment) => match b {
                StatusNodeSegment::LineSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
                StatusNodeSegment::ArcSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
            },
        });
        for segment in reinserted_segments {
            match segment {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    self.status_tree.insert(StatusNode {
                        value: calculate_segment_value(&line_segment, event_point),
                        point: event_point.clone(),
                        segment,
                    })
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    self.status_tree.insert(StatusNode {
                        value: calculate_segment_value(&arc_segment, event_point),
                        point: event_point.clone(),
                        segment,
                    })
                }
            }
        }
        let mid_order_traversal = self.status_tree.mid_order_traversal();
        if source_is_p_empty && contain_p_empty {
            let neighbors = self.get_neighbors_with_point(event_point);
            match neighbors {
                Some((segment_left, segment_right)) => {
                    self.find_intersection_points(&segment_left, &segment_right, event_point);
                }
                None => {}
            }
        } else {
            let segment_left = self
                .get_left_right_in_u_c(&source_is_p, &contain_p, event_point)
                .0;
            let segment_left_left = self.get_left_of_segment(&segment_left, &mid_order_traversal);
            match segment_left_left {
                Some(segment) => {
                    self.find_intersection_points(&segment_left, &segment, event_point);
                }
                _ => {}
            }
            let segment_right = self
                .get_left_right_in_u_c(&source_is_p, &contain_p, event_point)
                .1;
            let segment_right_right =
                self.get_right_of_segment(&segment_right, &mid_order_traversal);
            match segment_right_right {
                Some(segment) => {
                    self.find_intersection_points(&segment_right, &segment, event_point);
                }
                _ => {}
            }
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

    fn get_neighbors_with_point(
        &self,
        point: &Point2<T>,
    ) -> Option<(StatusNodeSegment<T>, StatusNodeSegment<T>)> {
        let status_nodes = self.status_tree.mid_order_traversal();
        let mut index = 0;
        let mut flag = false;
        for (status_index, status_node) in status_nodes.iter().enumerate() {
            if status_node.value.equals(point.y()) || status_node.value > point.y() {
                index = status_index;
                flag = true;
                break;
            }
        }
        if flag {
            if index == 0 {
                return None;
            } else {
                return Some((
                    status_nodes[index - 1].segment.clone(),
                    status_nodes[index].segment.clone(),
                ));
            }
        } else {
            return None;
        }
    }

    fn get_left_right_in_u_c(
        &self,
        source_is_p: &Vec<StatusNodeSegment<T>>,
        contain_p: &Vec<StatusNodeSegment<T>>,
        event_point: &Point2<T>,
    ) -> (StatusNodeSegment<T>, StatusNodeSegment<T>) {
        let mut segments = Vec::new();
        for segment in source_is_p {
            segments.push(segment.clone());
        }
        for segment in contain_p {
            segments.push(segment.clone());
        }
        segments.sort_by(|a, b| match a {
            StatusNodeSegment::LineSegment2(segment) => match b {
                StatusNodeSegment::LineSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
                StatusNodeSegment::ArcSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
            },
            StatusNodeSegment::ArcSegment2(segment) => match b {
                StatusNodeSegment::LineSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
                StatusNodeSegment::ArcSegment2(other_segment) => {
                    compare_segments(segment, other_segment, &event_point.clone())
                }
            },
        });
        let left = segments[0].clone();
        let right = segments[segments.len() - 1].clone();
        (left, right)
    }

    fn get_left_of_segment(
        &self,
        segment: &StatusNodeSegment<T>,
        mid_order_traversal: &Vec<StatusNode<T>>,
    ) -> Option<StatusNodeSegment<T>> {
        for (index, status_node) in mid_order_traversal.iter().enumerate() {
            let mut status_node_segment = status_node.segment.clone();
            match status_node_segment {
                StatusNodeSegment::LineSegment2(line_segment) => match segment {
                    StatusNodeSegment::LineSegment2(segment) => {
                        if line_segment.source().equals(&segment.source())
                            && line_segment.target().equals(&segment.target())
                        {
                            if index == 0 {
                                return None;
                            }
                            status_node_segment = mid_order_traversal[index - 1].segment.clone();
                            return Some(status_node_segment);
                        }
                    }
                    _ => {}
                },
                StatusNodeSegment::ArcSegment2(arc_segment) => match segment {
                    StatusNodeSegment::ArcSegment2(segment) => {
                        if arc_segment.center().equals(&segment.center())
                            && arc_segment.radius().equals(segment.radius())
                            && arc_segment.source().equals(&segment.source())
                            && arc_segment.target().equals(&segment.target())
                            && arc_segment.orientation() == segment.orientation()
                        {
                            if index == 0 {
                                return None;
                            }
                            status_node_segment = mid_order_traversal[index - 1].segment.clone();
                            return Some(status_node_segment);
                        }
                    }
                    _ => {}
                },
            }
        }
        None
    }

    fn get_right_of_segment(
        &self,
        segment: &StatusNodeSegment<T>,
        mid_order_traversal: &Vec<StatusNode<T>>,
    ) -> Option<StatusNodeSegment<T>> {
        for (index, status_node) in mid_order_traversal.iter().enumerate() {
            let mut status_node_segment = status_node.segment.clone();
            match status_node_segment {
                StatusNodeSegment::LineSegment2(line_segment) => match segment {
                    StatusNodeSegment::LineSegment2(segment) => {
                        if line_segment.source().equals(&segment.source())
                            && line_segment.target().equals(&segment.target())
                        {
                            if index == mid_order_traversal.len() - 1 {
                                return None;
                            }
                            status_node_segment = mid_order_traversal[index + 1].segment.clone();
                            return Some(status_node_segment);
                        }
                    }
                    _ => {}
                },
                StatusNodeSegment::ArcSegment2(arc_segment) => match segment {
                    StatusNodeSegment::ArcSegment2(segment) => {
                        if arc_segment.center().equals(&segment.center())
                            && arc_segment.radius().equals(segment.radius())
                            && arc_segment.source().equals(&segment.source())
                            && arc_segment.target().equals(&segment.target())
                            && arc_segment.orientation() == segment.orientation()
                        {
                            if index == mid_order_traversal.len() - 1 {
                                return None;
                            }
                            status_node_segment = mid_order_traversal[index + 1].segment.clone();
                            return Some(status_node_segment);
                        }
                    }
                    _ => {}
                },
            }
        }
        None
    }

    fn find_intersection_points(
        &mut self,
        segment_a: &StatusNodeSegment<T>,
        segment_b: &StatusNodeSegment<T>,
        event_point: &Point2<T>,
    ) {
        let points = match segment_a {
            StatusNodeSegment::LineSegment2(segment_a) => match segment_b {
                StatusNodeSegment::LineSegment2(segment_b) => {
                    segment_2_segment_2_intersection(segment_a, segment_b)
                }
                StatusNodeSegment::ArcSegment2(segment_b) => {
                    segment_2_segment_2_intersection(segment_a, segment_b)
                }
            },
            StatusNodeSegment::ArcSegment2(segment_a) => match segment_b {
                StatusNodeSegment::LineSegment2(segment_b) => {
                    segment_2_segment_2_intersection(segment_a, segment_b)
                }
                StatusNodeSegment::ArcSegment2(segment_b) => {
                    segment_2_segment_2_intersection(segment_a, segment_b)
                }
            },
        };
        for point in points {
            if point.x() > event_point.x()
                || (point.x().equals(event_point.x()) && point.y() > event_point.y())
            {
                self.event_queue.push(point);
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

fn calculate_segment_value<T: NumberType>(segment: &impl Segment2<T>, point: &Point2<T>) -> T {
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

fn compare_segments<T: NumberType>(
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
                    if segment_a.segment_type() == segment_b.segment_type() {
                        if segment_a.segment_type() == Segment2Type::LineSegment2 {
                            let target_a_y = segment_a.target().y();
                            let target_b_y = segment_b.target().y();
                            if target_a_y.equals(target_b_y) {
                                return std::cmp::Ordering::Equal;
                            } else if target_a_y < target_b_y {
                                return std::cmp::Ordering::Less;
                            } else {
                                return std::cmp::Ordering::Greater;
                            }
                        } else {
                            let mid_a = segment_a.source()+segment_a
                            return std::cmp::Ordering::Equal;
                        }
                    } else {
                        if segment_a.segment_type() != Segment2Type::LineSegment2 {
                            return std::cmp::Ordering::Less;
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
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
            let mut point = self.point.clone();
            if point > other.point {
                point = other.point.clone();
            }
            return match self.segment {
                StatusNodeSegment::LineSegment2(segment) => match other.segment {
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
                StatusNodeSegment::ArcSegment2(segment) => match other.segment {
                    StatusNodeSegment::LineSegment2(other_segment) => {
                        compare_segments(&segment, &other_segment, &point)
                    }
                    StatusNodeSegment::ArcSegment2(other_segment) => {
                        if segment.center().equals(&other_segment.center())
                            && segment.radius().equals(other_segment.radius())
                            && segment.source().equals(&other_segment.source())
                            && segment.target().equals(&other_segment.target())
                            && segment.orientation() == other_segment.orientation()
                        {
                            std::cmp::Ordering::Equal
                        } else {
                            compare_segments(&segment, &other_segment, &point)
                        }
                    }
                },
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sweep_line_segment_2_intersection() {
        let circle_segment = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        let mut sweep = SweepSegment2Intersection::new();
        sweep.push_segment(&LineSegment2::new(
            Point2::new(-5.0, 5.0),
            Point2::new(5.0, -5.0),
        ));

        sweep.push_segment(&ArcSegment2::new(
            circle_segment.clone(),
            0.0,
            std::f64::consts::PI,
        ));

        sweep.push_segment(&ArcSegment2::new(
            circle_segment.clone(),
            std::f64::consts::PI,
            std::f64::consts::PI * 2.0,
        ));
        let result = sweep.intersection();
        println!("{:?}", result);
    }
}
