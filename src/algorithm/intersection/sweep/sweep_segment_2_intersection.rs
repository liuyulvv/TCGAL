use super::sweep_event_queue::SweepEventQueue;
use super::sweep_status_node::SweepStatusNode;
use super::sweep_status_segment::StatusNodeSegment;
use super::sweep_status_tree::SweepStatusTree;
use super::sweep_util::calculate_segment_value;
use crate::algorithm::intersection::segment_2_segment_2::segment_2_segment_2_intersection;
use crate::algorithm::location::{
    point_2_arc_segment_2::is_point_2_on_arc_segment_2,
    point_2_line_segment_2::is_point_2_on_line_segment_2,
};
use crate::data_structure::avl_tree::{AVLTree, AVLTreeOption};
use crate::kernel::{
    arc_segment_2::ArcSegment2, circle_segment_2::CircleSegment2, line_segment_2::LineSegment2,
    number_type::NumberType, point_2::Point2, segment_2::Segment2, util_enum::Segment2Type,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct SweepSegment2Intersection<T: NumberType> {
    origin_segments: Vec<StatusNodeSegment<T>>,
    status_nodes: Vec<Rc<RefCell<SweepStatusNode<T>>>>,
    event_queue: SweepEventQueue<T>,
    status_tree: SweepStatusTree<T>,
    last_event_point: Option<Point2<T>>,
    intersection_points: AVLTree<Point2<T>>,
}

impl<T: NumberType> SweepSegment2Intersection<T> {
    pub fn new() -> Self {
        Self {
            origin_segments: Vec::new(),
            status_nodes: Vec::new(),
            event_queue: SweepEventQueue::new(),
            status_tree: SweepStatusTree::new(),
            last_event_point: None,
            intersection_points: AVLTree::new(AVLTreeOption::DisableSameNode),
        }
    }

    pub fn push_segment(&mut self, segment: &impl Segment2<T>) {
        match segment.segment_type() {
            Segment2Type::LineSegment2 => {
                let source = segment.source();
                let target = segment.target();
                self.origin_segments
                    .push(StatusNodeSegment::LineSegment2(LineSegment2::new(
                        source, target,
                    )));
                if source > target {
                    self.status_nodes
                        .push(Rc::new(RefCell::new(SweepStatusNode {
                            value: source.y(),
                            point: source.clone(),
                            segment: Rc::new(RefCell::new(StatusNodeSegment::LineSegment2(
                                LineSegment2::new(source, target),
                            ))),
                            left: None,
                            right: None,
                        })));
                } else {
                    self.status_nodes
                        .push(Rc::new(RefCell::new(SweepStatusNode {
                            value: target.y(),
                            point: target.clone(),
                            segment: Rc::new(RefCell::new(StatusNodeSegment::LineSegment2(
                                LineSegment2::new(target, source),
                            ))),
                            left: None,
                            right: None,
                        })));
                }
            }
            Segment2Type::CircleSegment2 => {
                let circle_segment = CircleSegment2::new(segment.center(), segment.radius());
                self.origin_segments
                    .push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
                        circle_segment.clone(),
                        T::zero(),
                        T::pi() * T::from_f64(2.0),
                    )));

                let center = segment.center();
                let radius = segment.radius();

                let left_point = Point2::new(center.x() - radius, center.y());
                let right_point = Point2::new(center.x() + radius, center.y());

                self.status_nodes
                    .push(Rc::new(RefCell::new(SweepStatusNode {
                        value: center.y(),
                        point: left_point,
                        segment: Rc::new(RefCell::new(StatusNodeSegment::ArcSegment2(
                            ArcSegment2::new(circle_segment.clone(), T::zero(), T::pi()),
                        ))),
                        left: None,
                        right: None,
                    })));

                self.status_nodes
                    .push(Rc::new(RefCell::new(SweepStatusNode {
                        value: center.y(),
                        point: right_point,
                        segment: Rc::new(RefCell::new(StatusNodeSegment::ArcSegment2(
                            ArcSegment2::new(
                                circle_segment.clone(),
                                T::pi(),
                                T::pi() * T::from_f64(2.0),
                            ),
                        ))),
                        left: None,
                        right: None,
                    })));
            }
            Segment2Type::ArcSegment2 => {
                let center = segment.center();
                let radius = segment.radius();
                let circle_segment = CircleSegment2::new(center, radius);

                let arc_segment = ArcSegment2::new(
                    circle_segment.clone(),
                    segment.source_radian(),
                    segment.target_radian(),
                );
                self.origin_segments
                    .push(StatusNodeSegment::ArcSegment2(ArcSegment2::new(
                        circle_segment.clone(),
                        segment.source_radian(),
                        segment.target_radian(),
                    )));
                let arc_segments = arc_segment.monotone();
                for arc_segment in arc_segments {
                    self.status_nodes
                        .push(Rc::new(RefCell::new(SweepStatusNode {
                            value: arc_segment.source().y(),
                            point: arc_segment.source(),
                            segment: Rc::new(RefCell::new(StatusNodeSegment::ArcSegment2(
                                arc_segment,
                            ))),
                            left: None,
                            right: None,
                        })));
                }
            }
        }
    }

    pub fn intersection(&mut self) -> Vec<Point2<T>> {
        self.event_queue.clear();
        self.status_tree.clear();
        self.intersection_points.clear();
        let mut event_points: AVLTree<Point2<T>> = AVLTree::new(AVLTreeOption::DisableSameNode);
        for node in &self.status_nodes {
            match *node.borrow().segment.borrow() {
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
        let points = self.intersection_points.mid_order_traversal();
        self.filter_intersection_points(points)
    }

    fn handle_event_point(&mut self, event_point: &Point2<T>) {
        let source_is_p = self.get_segment_with_source(event_point);
        let (target_is_p, contain_p) = self.get_active_segment_target_with_and_contain(event_point);
        if source_is_p.is_empty() && target_is_p.is_empty() && contain_p.is_empty() {
            return;
        }
        if source_is_p.len() + target_is_p.len() + contain_p.len() > 1 {
            self.intersection_points.insert(event_point.clone());
        }
        for node in target_is_p {
            self.status_tree.delete(node.clone());
        }
        for node in &contain_p {
            self.status_tree.delete(node.clone())
        }
        let mut reinserted_nodes = Vec::new();
        for node in source_is_p {
            reinserted_nodes.push(node.clone());
        }
        for node in contain_p {
            reinserted_nodes.push(node.clone());
        }
        reinserted_nodes.sort();
        for node in &reinserted_nodes {
            {
                let mut node_borrow = node.borrow_mut();
                let value = {
                    let segment = node_borrow.segment.borrow_mut();
                    match *segment {
                        StatusNodeSegment::LineSegment2(ref line_segment) => {
                            calculate_segment_value(line_segment, event_point)
                        }
                        StatusNodeSegment::ArcSegment2(ref arc_segment) => {
                            calculate_segment_value(arc_segment, event_point)
                        }
                    }
                };
                node_borrow.value = value;
            }
            self.status_tree.insert(node.clone());
        }
        if reinserted_nodes.is_empty() {
            let neighbors = self.get_neighbors_with_point(event_point);
            match neighbors {
                Some((segment_left, segment_right)) => {
                    self.find_intersection_points(segment_left, segment_right, event_point);
                }
                None => {}
            }
        } else {
            let segment_left = reinserted_nodes[0].clone();
            let segment_right = reinserted_nodes[reinserted_nodes.len() - 1].clone();

            let left = segment_left.borrow().left.clone();
            let right = segment_right.borrow().right.clone();

            match left {
                Some(left) => {
                    self.find_intersection_points(segment_left.clone(), left, event_point)
                }
                None => {}
            }

            match right {
                Some(right) => {
                    self.find_intersection_points(segment_right.clone(), right, event_point)
                }
                None => {}
            }
        }
    }

    fn get_segment_with_source(
        &self,
        event_point: &Point2<T>,
    ) -> Vec<Rc<RefCell<SweepStatusNode<T>>>> {
        let mut result = Vec::new();
        for node in &self.status_nodes {
            match *node.borrow().segment.borrow() {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    if line_segment.source().equals(event_point) {
                        result.push(node.clone());
                    }
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    if (arc_segment.is_top() && arc_segment.target().equals(event_point))
                        || (!arc_segment.is_top() && arc_segment.source().equals(event_point))
                    {
                        result.push(node.clone());
                    }
                }
            }
        }
        result
    }

    fn get_active_segment_target_with_and_contain(
        &self,
        point: &Point2<T>,
    ) -> (
        Vec<Rc<RefCell<SweepStatusNode<T>>>>,
        Vec<Rc<RefCell<SweepStatusNode<T>>>>,
    ) {
        let mut target_result = Vec::new();
        let mut contain_result = Vec::new();
        let status_nodes = self.status_tree.mid_order_traversal();
        for status_node in status_nodes {
            match *status_node.borrow().segment.borrow() {
                StatusNodeSegment::LineSegment2(line_segment) => {
                    if line_segment.target().equals(point) {
                        target_result.push(status_node.clone());
                    } else {
                        let source = line_segment.source();
                        let target = line_segment.target();
                        if source.equals(point) || target.equals(point) {
                            continue;
                        }
                        if is_point_2_on_line_segment_2(point, &line_segment) {
                            contain_result.push(status_node.clone());
                        }
                    }
                }
                StatusNodeSegment::ArcSegment2(arc_segment) => {
                    if (arc_segment.is_top() && arc_segment.source().equals(point))
                        || (!arc_segment.is_top() && arc_segment.target().equals(point))
                    {
                        target_result.push(status_node.clone());
                    } else {
                        let source = arc_segment.source();
                        let target = arc_segment.target();
                        if source.equals(point) || target.equals(point) {
                            continue;
                        }
                        if is_point_2_on_arc_segment_2(point, &arc_segment) {
                            contain_result.push(status_node.clone());
                        }
                    }
                }
            }
        }
        (target_result, contain_result)
    }

    fn get_neighbors_with_point(
        &self,
        point: &Point2<T>,
    ) -> Option<(
        Rc<RefCell<SweepStatusNode<T>>>,
        Rc<RefCell<SweepStatusNode<T>>>,
    )> {
        let status_nodes = self.status_tree.mid_order_traversal();
        let mut index = 0;
        let mut flag = false;
        for (status_index, status_node) in status_nodes.iter().enumerate() {
            let node = status_node.borrow();
            if node.value.equals(point.y()) || node.value > point.y() {
                index = status_index;
                flag = true;
                break;
            }
        }
        if flag {
            if index == 0 {
                return None;
            } else {
                return Some((status_nodes[index - 1].clone(), status_nodes[index].clone()));
            }
        } else {
            return None;
        }
    }

    fn find_intersection_points(
        &mut self,
        segment_a: Rc<RefCell<SweepStatusNode<T>>>,
        segment_b: Rc<RefCell<SweepStatusNode<T>>>,
        event_point: &Point2<T>,
    ) {
        let points = match *segment_a.borrow().segment.borrow() {
            StatusNodeSegment::LineSegment2(segment_a) => {
                match *segment_b.borrow().segment.borrow() {
                    StatusNodeSegment::LineSegment2(segment_b) => {
                        segment_2_segment_2_intersection(&segment_a, &segment_b)
                    }
                    StatusNodeSegment::ArcSegment2(segment_b) => {
                        segment_2_segment_2_intersection(&segment_a, &segment_b)
                    }
                }
            }
            StatusNodeSegment::ArcSegment2(segment_a) => match *segment_b.borrow().segment.borrow()
            {
                StatusNodeSegment::LineSegment2(segment_b) => {
                    segment_2_segment_2_intersection(&segment_a, &segment_b)
                }
                StatusNodeSegment::ArcSegment2(segment_b) => {
                    segment_2_segment_2_intersection(&segment_a, &segment_b)
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

    fn filter_intersection_points(&self, points: Vec<Point2<T>>) -> Vec<Point2<T>> {
        let mut result = Vec::new();
        for point in points {
            let mut sum = 0;
            for segment in &self.origin_segments {
                match segment {
                    StatusNodeSegment::LineSegment2(line_segment) => {
                        if is_point_2_on_line_segment_2(&point, line_segment) {
                            sum += 1;
                        }
                    }
                    StatusNodeSegment::ArcSegment2(arc_segment) => {
                        if is_point_2_on_arc_segment_2(&point, arc_segment) {
                            sum += 1;
                        }
                    }
                }
                if sum > 1 {
                    result.push(point);
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sweep_line_segment_2_intersection() {
        let mut sweep = SweepSegment2Intersection::new();
        sweep.push_segment(&LineSegment2::new(
            Point2::new(10.0, 10.0),
            Point2::new(0.0, 10.0),
        ));
        sweep.push_segment(&LineSegment2::new(
            Point2::new(0.0, 5.0),
            Point2::new(5.0, 10.0),
        ));
        sweep.push_segment(&LineSegment2::new(
            Point2::new(3.0, 0.0),
            Point2::new(3.0, 15.0),
        ));
        sweep.push_segment(&LineSegment2::new(
            Point2::new(3.0, 8.0),
            Point2::new(10.0, 10.0),
        ));
        sweep.push_segment(&LineSegment2::new(
            Point2::new(3.0, 12.0),
            Point2::new(5.0, 0.0),
        ));

        let result = sweep.intersection();
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

        // let circle_segment = CircleSegment2::new(Point2::new(0.0, 0.0), 5.0);
        // let mut sweep = SweepSegment2Intersection::new();
        // sweep.push_segment(&LineSegment2::new(
        //     Point2::new(-5.0, 5.0),
        //     Point2::new(5.0, -5.0),
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     circle_segment.clone(),
        //     0.0,
        //     std::f64::consts::PI,
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     circle_segment.clone(),
        //     std::f64::consts::PI,
        //     std::f64::consts::PI * 2.0,
        // ));
        // let result = sweep.intersection();
        // assert_eq!(
        //     result,
        //     vec![
        //         Point2::new(5.0, 0.0),
        //         Point2::new(3.5355339059327373, -3.5355339059327373),
        //         Point2::new(-3.5355339059327373, 3.5355339059327373),
        //         Point2::new(-5.0, 0.0),
        //     ]
        // );

        // let mut sweep = SweepSegment2Intersection::new();
        // sweep.push_segment(&LineSegment2::new(
        //     Point2::new(-5.0, 5.0),
        //     Point2::new(5.0, -5.0),
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     CircleSegment2::new(Point2::new(0.0, 0.0), 3.0),
        //     0.0,
        //     std::f64::consts::PI,
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     CircleSegment2::new(Point2::new(0.0, -3.0), 5.0),
        //     0.0,
        //     std::f64::consts::PI,
        // ));
        // let result = sweep.intersection();
        // assert_eq!(
        //     result,
        //     vec![
        //         Point2::new(2.7638539919628324, 1.166666666666667),
        //         Point2::new(-1.7015621187164243, 1.7015621187164243),
        //         Point2::new(-2.1213203435596424, 2.1213203435596424),
        //         Point2::new(-2.7638539919628333, 1.1666666666666667)
        //     ]
        // );

        // let mut sweep = SweepSegment2Intersection::new();
        // sweep.push_segment(&LineSegment2::new(
        //     Point2::new(-5.0, 5.0),
        //     Point2::new(5.0, -5.0),
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     CircleSegment2::new(Point2::new(0.0, 4.0), 2.0),
        //     1.5 * std::f64::consts::PI,
        //     3.0 * std::f64::consts::PI,
        // ));

        // sweep.push_segment(&ArcSegment2::new(
        //     CircleSegment2::new(Point2::new(0.0, -3.0), 5.0),
        //     0.0,
        //     std::f64::consts::PI,
        // ));
        // let result = sweep.intersection();
        // assert_eq!(
        //     result,
        //     vec![
        //         Point2::new(0.0, 2.0),
        //         Point2::new(-1.7015621187164243, 1.7015621187164243),
        //     ]
        // );
    }
}
