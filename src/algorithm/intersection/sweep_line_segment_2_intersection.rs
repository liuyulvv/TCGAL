use crate::{
    algorithm::{
        intersection::segment_2_segment_2::segment_2_segment_2_intersection,
        location::point_2_segment_2::is_point_2_on_segment_2,
    },
    data_structure::{
        avl_tree::{AVLTree, AVLTreeOption},
        priority_queue::PriorityQueue,
    },
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
    intersection_points: AVLTree<Point2<T>>,
    last_event_point: Option<Point2<T>>,
}

impl<T: NumberType> SweepLineSegment2Intersection<T> {
    pub fn new(input_segments: &Vec<Segment2<T>>) -> Self {
        let mut event_queue = PriorityQueue::new();
        let mut segments = Vec::new();
        let status_tree: AVLTree<StatusNode<T>> = AVLTree::new(AVLTreeOption::SameNodeInsertRight);
        let intersection_points = AVLTree::new(AVLTreeOption::DisableSameNode);
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
            last_event_point: None,
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
                    Some(value) => self.calculate_segment_value(segment, &value),
                    None => segment.source().y(),
                },
                segment: segment.clone(),
            });
        }
        for segment in &contain_p {
            self.status_tree.delete(StatusNode {
                value: match self.last_event_point {
                    Some(value) => self.calculate_segment_value(segment, &value),
                    None => segment.source().y(),
                },
                segment: segment.clone(),
            });
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
        reinserted_segments.sort_by(|a, b| {
            let a_value = self.calculate_segment_value(a, &event_point);
            let b_value = self.calculate_segment_value(b, &event_point);
            if a_value.equals(b_value) {
                let a_target_y = a.target().y();
                let b_target_y = b.target().y();
                if a_target_y.equals(b_target_y) {
                    let a_target_x = a.target().x();
                    let b_target_x = b.target().x();
                    if a_target_x < b_target_x {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                } else if a_target_y < b_target_y {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            } else if a_value < b_value {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        for segment in reinserted_segments {
            self.status_tree.insert(StatusNode {
                value: self.calculate_segment_value(&segment, &event_point),
                segment,
            });
        }

        let mid_order_traversal = self.status_tree.mid_order_traversal();
        if source_is_p_empty && contain_p_empty {
            let neighbors = self.get_neighbors_with_point(event_point);
            match neighbors {
                Some((segment_left, segment_right)) => {
                    self.find_intersection_points(&segment_left, &segment_right, &event_point);
                }
                None => {}
            }
        } else {
            let segment_left = self
                .get_left_right_in_u_c(&source_is_p, &contain_p, &event_point)
                .0;
            let segment_left_left = self.get_left_of_segment(&segment_left, &mid_order_traversal);
            match segment_left_left {
                Some(segment) => {
                    self.find_intersection_points(&segment_left, &segment, &event_point);
                }
                None => {}
            }
            let segment_right = self
                .get_left_right_in_u_c(&source_is_p, &contain_p, &event_point)
                .1;
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
            if status_node.value >= point.y() {
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
        source_is_p: &Vec<Segment2<T>>,
        contain_p: &Vec<Segment2<T>>,
        event_point: &Point2<T>,
    ) -> (Segment2<T>, Segment2<T>) {
        let mut segments = Vec::new();
        for segment in source_is_p {
            segments.push(segment.clone());
        }
        for segment in contain_p {
            segments.push(segment.clone());
        }
        segments.sort_by(|a, b| {
            let a_value = self.calculate_segment_value(a, &event_point);
            let b_value = self.calculate_segment_value(b, &event_point);
            if a_value.equals(b_value) {
                let a_target_y = a.target().y();
                let b_target_y = b.target().y();
                if a_target_y.equals(b_target_y) {
                    let a_target_x = a.target().x();
                    let b_target_x = b.target().x();
                    if a_target_x < b_target_x {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                } else if a_target_y < b_target_y {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            } else if a_value < b_value {
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
            if point.x() > event_point.x()
                || (point.x().equals(event_point.x()) && point.y() > event_point.y())
            {
                self.event_queue.push(point)
            }
        }
    }

    fn calculate_segment_value(&self, segment: &Segment2<T>, point: &Point2<T>) -> T {
        let source = segment.source();
        let target = segment.target();
        if source.x().equals(target.x()) {
            return point.y();
        }
        let y = source.y()
            + (point.x() - source.x()) * (target.y() - source.y()) / (target.x() - source.x());
        y
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
        if self.value.equals(other.value) {
            let source = self.segment.source();
            let target = self.segment.target();
            let other_source = other.segment.source();
            let other_target = other.segment.target();
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
        let segment1 = Segment2::new(Point2::new(10.0, 10.0), Point2::new(0.0, 10.0));
        let segment2 = Segment2::new(Point2::new(0.0, 5.0), Point2::new(5.0, 10.0));
        let segment3 = Segment2::new(Point2::new(3.0, 0.0), Point2::new(3.0, 15.0));
        let segment4 = Segment2::new(Point2::new(3.0, 8.0), Point2::new(10.0, 10.0));
        let segment5 = Segment2::new(Point2::new(3.0, 12.0), Point2::new(5.0, 0.0));
        let segments = vec![segment1, segment2, segment3, segment4, segment5];
        let mut sweep_line = SweepLineSegment2Intersection::new(&segments);
        let result = sweep_line.intersection();
        assert_eq!(
            result,
            vec![
                Point2::new(10.0, 10.0),
                Point2::new(5.0, 10.0),
                Point2::new(3.636363636363636, 8.181818181818182),
                Point2::new(3.571428571428571, 8.571428571428571),
                Point2::new(3.3333333333333335, 10.0),
                Point2::new(3.0, 12.0),
                Point2::new(3.0, 10.0),
                Point2::new(3.0, 8.0),
            ]
        );
    }
}
