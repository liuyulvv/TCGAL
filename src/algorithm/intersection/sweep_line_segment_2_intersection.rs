use std::collections::HashSet;

use crate::{
    algorithm::{
        intersection::segment_2_segment_2::segment_2_segment_2_intersection,
        location::point_2_segment_2::is_point_2_on_segment_2,
    },
    data_structure::{avl_tree::AVLTree, priority_queue::PriorityQueue},
    kernel::{number_type::NumberType, point_2::Point2, segment_2::Segment2},
};

#[derive(Debug, Clone, Copy)]
struct StatusNode<T: NumberType> {
    value: T,
    segment: Segment2<T>,
}

pub struct SweepLineSegment2Intersection<T: NumberType> {
    segments: Vec<Segment2<T>>,
    event_queue: PriorityQueue<Point2<T>>,
    status_tree: AVLTree<StatusNode<T>>,
    intersection_points: Vec<Point2<T>>,
    last_status_value: Option<T>,
}

impl<T: NumberType> SweepLineSegment2Intersection<T> {
    pub fn new(input_segments: &Vec<Segment2<T>>) -> Self {
        let mut event_queue = PriorityQueue::new();
        let mut segments = Vec::new();
        let status_tree: AVLTree<StatusNode<T>> = AVLTree::new();
        let intersection_points = Vec::new();
        for segment in input_segments {
            let source = segment.source();
            let target = segment.target();
            if source > target {
                segments.push(Segment2::new(source, target));
            } else {
                segments.push(Segment2::new(target, source));
            }
        }
        for segment in &segments {
            event_queue.push(segment.source());
            event_queue.push(segment.target());
        }
        Self {
            segments,
            event_queue,
            status_tree,
            intersection_points,
            last_status_value: None,
        }
    }

    pub fn intersection(&mut self) -> Vec<Point2<T>> {
        self.event_queue.clear();
        self.status_tree.clear();
        self.intersection_points.clear();
        for segment in &self.segments {
            self.event_queue.push(segment.source());
            self.event_queue.push(segment.target());
        }
        while !self.event_queue.is_empty() {
            let event_point = self.event_queue.pop().unwrap();
            self.handle_event_point(&event_point);
            self.last_status_value = Some(event_point.y());
        }
        self.intersection_points.iter().cloned().collect()
    }

    fn handle_event_point(&mut self, event_point: &Point2<T>) {
        let u_p = self.get_segment_with_source(event_point);
        let l_p = self.get_active_segment_with_target(event_point);
        let c_p = self.get_active_segment_containing_point(event_point);
        if u_p.is_empty() && l_p.is_empty() && c_p.is_empty() {
            return;
        }
        if u_p.len() + l_p.len() + c_p.len() > 1 {
            self.intersection_points.push(event_point.clone());
        }
        for segment in &l_p {
            self.status_tree.delete(StatusNode {
                value: match self.last_status_value {
                    Some(value) => self.calculate_segment_value(segment, value),
                    None => segment.source().x(),
                },
                segment: segment.clone(),
            });
        }
        for segment in &c_p {
            self.status_tree.delete(StatusNode {
                value: match self.last_status_value {
                    Some(value) => self.calculate_segment_value(segment, value),
                    None => segment.source().x(),
                },
                segment: segment.clone(),
            });
        }
        let u_p_empty = u_p.is_empty();
        let c_p_empty = c_p.is_empty();

        let mid_order_traversal = self.status_tree.mid_order_traversal();
        self.status_tree.clear();
        let mut reinserted_segments = Vec::new();
        for segment in &mid_order_traversal {
            reinserted_segments.push(segment.segment.clone());
        }
        for segment in &u_p {
            reinserted_segments.push(segment.clone());
        }
        for segment in &c_p {
            reinserted_segments.push(segment.clone());
        }
        reinserted_segments.sort_by(|a, b| {
            let a_target_x = a.target().x();
            let b_target_x = b.target().x();
            if a_target_x < b_target_x {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        for segment in reinserted_segments {
            self.status_tree.insert(StatusNode {
                value: self.calculate_segment_value(&segment, event_point.y()),
                segment,
            });
        }
        let mid_order_traversal = self.status_tree.mid_order_traversal();
        if u_p_empty && c_p_empty {
            let neighbors = self.get_neighbors_with_point(event_point);
            match neighbors {
                Some((segment_left, segment_right)) => {
                    self.find_intersection_points(&segment_left, &segment_right, &event_point);
                }
                None => {}
            }
        } else {
            let segment_left = self.get_left_right_in_u_c(&u_p, &c_p).0;
            let segment_left_left = self.get_left_of_segment(&segment_left, &mid_order_traversal);
            match segment_left_left {
                Some(segment) => {
                    self.find_intersection_points(&segment_left, &segment, &event_point);
                }
                None => {}
            }
            let segment_right = self.get_left_right_in_u_c(&u_p, &c_p).1;
            let segment_right_right =
                self.get_right_of_segment(&segment_right, &mid_order_traversal);
            match segment_right_right {
                Some(segment) => {
                    self.find_intersection_points(&segment_right, &segment, &event_point);
                }
                None => {}
            }
        }
    }

    fn get_segment_with_source(&self, source: &Point2<T>) -> Vec<Segment2<T>> {
        let mut result = Vec::new();
        for segment in &self.segments {
            if segment.source().equals(source) {
                result.push(*segment);
            }
        }
        result
    }

    fn get_active_segment_with_target(&self, target: &Point2<T>) -> Vec<Segment2<T>> {
        let mut result = Vec::new();
        let status_nodes = self.status_tree.mid_order_traversal();
        for status_node in status_nodes {
            if status_node.segment.target().equals(target) {
                result.push(status_node.segment);
            }
        }
        result
    }

    fn get_active_segment_containing_point(&self, point: &Point2<T>) -> Vec<Segment2<T>> {
        let mut result = Vec::new();
        let status_nodes = self.status_tree.mid_order_traversal();
        for status_node in status_nodes {
            let start = status_node.segment.source();
            let target = status_node.segment.target();
            if start.equals(point) || target.equals(point) {
                continue;
            }
            if is_point_2_on_segment_2(point, &status_node.segment) {
                result.push(status_node.segment);
            }
        }
        result
    }

    fn get_neighbors_with_point(&self, point: &Point2<T>) -> Option<(Segment2<T>, Segment2<T>)> {
        let status_nodes = self.status_tree.mid_order_traversal();
        let mut index = 0;
        let mut flag = false;
        for (status_index, status_node) in status_nodes.iter().enumerate() {
            if status_node.value >= point.x() {
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
        u_p: &Vec<Segment2<T>>,
        c_p: &Vec<Segment2<T>>,
    ) -> (Segment2<T>, Segment2<T>) {
        let mut segments = Vec::new();
        for segment in u_p {
            segments.push(segment.clone());
        }
        for segment in c_p {
            segments.push(segment.clone());
        }
        segments.sort_by(|a, b| {
            let a_target_x = a.target().x();
            let b_target_x = b.target().x();
            if a_target_x < b_target_x {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        let left = segments[0].clone();
        let right = segments[segments.len() - 1].clone();
        (left, right)
    }

    fn get_left_of_segment(
        &self,
        segment: &Segment2<T>,
        mid_order_traversal: &Vec<StatusNode<T>>,
    ) -> Option<Segment2<T>> {
        for (index, status_node) in mid_order_traversal.iter().enumerate() {
            let mut status_segment = status_node.segment.clone();
            if status_segment.source().equals(&segment.source())
                && status_segment.target().equals(&segment.target())
            {
                if index == 0 {
                    return None;
                }
                status_segment = mid_order_traversal[index - 1].segment.clone();
                return Some(status_segment);
            }
        }
        None
    }

    fn get_right_of_segment(
        &self,
        segment: &Segment2<T>,
        mid_order_traversal: &Vec<StatusNode<T>>,
    ) -> Option<Segment2<T>> {
        for (index, status_node) in mid_order_traversal.iter().enumerate() {
            let mut status_segment = status_node.segment.clone();
            if status_segment.source().equals(&segment.source())
                && status_segment.target().equals(&segment.target())
            {
                if index == mid_order_traversal.len() - 1 {
                    return None;
                }
                status_segment = mid_order_traversal[index + 1].segment.clone();
                return Some(status_segment);
            }
        }
        None
    }

    fn find_intersection_points(
        &mut self,
        s1: &Segment2<T>,
        s2: &Segment2<T>,
        event_point: &Point2<T>,
    ) {
        let points = segment_2_segment_2_intersection(s1, s2);
        for point in points {
            if point.y() < event_point.y()
                || (point.y().equals(event_point.y()) && point.x() > event_point.x())
            {
                self.event_queue.push(point)
            }
        }
    }

    fn calculate_segment_value(&self, segment: &Segment2<T>, y: T) -> T {
        let source = segment.source();
        let target = segment.target();
        if source.y().equals(target.y()) {
            return target.x();
        }
        let x =
            source.x() + (y - source.y()) * (target.x() - source.x()) / (target.y() - source.y());
        x
    }
}

impl<T: NumberType> Eq for StatusNode<T> {}

impl<T: NumberType> PartialEq for StatusNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.segment.source().equals(&other.segment.source())
            && self.segment.target().equals(&other.segment.target())
    }
}

impl<T: NumberType> Ord for StatusNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value;
        let other_value = other.value;
        if self_value < other_value {
            return std::cmp::Ordering::Less;
        } else if self_value > other_value {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
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
        // let segment1 = Segment2::new(Point2::new(10.0, 10.0), Point2::new(0.0, 0.0));
        // let segment2 = Segment2::new(Point2::new(0.0, 10.0), Point2::new(10.0, 0.0));
        // let segment3 = Segment2::new(Point2::new(0.0, 5.0), Point2::new(8.0, 8.0));
        // let segment1 = Segment2::new(Point2::new(10.0, 10.0), Point2::new(0.0, 10.0));
        // let segment2 = Segment2::new(Point2::new(0.0, 5.0), Point2::new(5.0, 10.0));
        // let segment3 = Segment2::new(Point2::new(3.0, 0.0), Point2::new(3.0, 15.0));
        let segment1 = Segment2::new(Point2::new(0.0, 10.0), Point2::new(10.0, 0.0));
        let segment2 = Segment2::new(Point2::new(5.0, 10.0), Point2::new(5.0, 8.0));
        let segment3 = Segment2::new(Point2::new(10.0, 9.0), Point2::new(0.0, 0.0));
        let segments = vec![segment1, segment2, segment3];
        let mut sweep_line = SweepLineSegment2Intersection::new(&segments);
        let result = sweep_line.intersection();
        println!("{:?}", result);
        // assert_eq!(result.len(), 0);
    }
}
