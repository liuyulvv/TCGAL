use std::{cell::RefCell, rc::Rc};

use crate::{
    algorithm::location::{
        location_enum::Point2Triangle2Location, point_2_triangle_2::locate_point_2_triangle_2,
    },
    data_structure::circular_doubly_linked_list::{CircularDoubleLinkedList, ListNode},
    kernel::{
        number_type::NumberType, point_2::Point2, polygon_2::Polygon2, triangle_2::Triangle2,
        util_enum::Orientation,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VertexType {
    Convex,
    Reflex,
    Ear,
}

#[derive(Clone, Copy)]
struct EarcutVertex<T: NumberType> {
    pub index: usize,
    pub point: Point2<T>,
    pub vertex_type: VertexType,
}

pub fn earcut_2<T: NumberType>(polygon: Polygon2<T>) -> Vec<Vec<usize>> {
    let is_simple = polygon.is_simple();
    if !is_simple {
        panic!("Polygon is not simple");
    }
    let mut vertices = CircularDoubleLinkedList::new();
    let mut ears = Vec::new();
    let mut reflexes = Vec::new();
    let polygon_vertices = polygon.vertices();
    for i in 0..polygon_vertices.len() {
        let vertex_type = init_vertex_type(&polygon_vertices, i);
        let vertex = vertices.insert(
            vertices.tail(),
            EarcutVertex {
                index: i,
                point: polygon_vertices[i],
                vertex_type,
            },
        );
        match vertex_type {
            VertexType::Ear => ears.push(vertex),
            VertexType::Reflex => reflexes.push(vertex),
            _ => {}
        }
    }
    let mut triangles = Vec::new();

    while !ears.is_empty() && vertices.len() > 3 {
        let ear: Rc<RefCell<ListNode<EarcutVertex<T>>>> = ears.pop().unwrap();
        let prev = ear.borrow().prev.clone().unwrap();
        let next = ear.borrow().next.clone().unwrap();
        let ear_index = ear.borrow().data.index;
        let prev_index = prev.borrow().data.index;
        let next_index = next.borrow().data.index;
        let mut vec = vec![prev_index, ear_index, next_index];
        vec.sort();
        triangles.push(vec);
        vertices.delete(Some(ear));
        println!("{:?}", ear_index);
        if vertices.len() > 3 {
            let (prev_vertex_type, index) = get_vertex_type(&ears, &reflexes, prev.clone());
            match prev_vertex_type {
                VertexType::Convex => {
                    if prev.borrow().data.vertex_type == VertexType::Reflex {
                        match index {
                            Some(i) => {
                                reflexes.remove(i);
                            }
                            None => {}
                        }
                    } else if prev.borrow().data.vertex_type == VertexType::Ear {
                        match index {
                            Some(i) => {
                                ears.remove(i);
                            }
                            None => {}
                        }
                    }
                    prev.borrow_mut().data.vertex_type = prev_vertex_type;
                }
                VertexType::Ear => {
                    if prev.borrow().data.vertex_type == VertexType::Reflex {
                        match index {
                            Some(i) => {
                                reflexes.remove(i);
                            }
                            None => {}
                        }
                        prev.borrow_mut().data.vertex_type = prev_vertex_type;
                        ears.push(prev);
                    } else if prev.borrow().data.vertex_type == VertexType::Convex {
                        prev.borrow_mut().data.vertex_type = prev_vertex_type;
                        ears.push(prev);
                    }
                }
                VertexType::Reflex => {
                    if prev.borrow().data.vertex_type == VertexType::Ear {
                        match index {
                            Some(i) => {
                                ears.remove(i);
                            }
                            None => {}
                        }
                        prev.borrow_mut().data.vertex_type = prev_vertex_type;
                        reflexes.push(prev);
                    } else if prev.borrow().data.vertex_type == VertexType::Convex {
                        prev.borrow_mut().data.vertex_type = prev_vertex_type;
                        reflexes.push(prev);
                    }
                }
            }
            let (next_vertex_type, index) = get_vertex_type(&ears, &reflexes, next.clone());
            match next_vertex_type {
                VertexType::Convex => {
                    if next.borrow().data.vertex_type == VertexType::Reflex {
                        match index {
                            Some(i) => {
                                reflexes.remove(i);
                            }
                            None => {}
                        }
                    } else if next.borrow().data.vertex_type == VertexType::Ear {
                        match index {
                            Some(i) => {
                                ears.remove(i);
                            }
                            None => {}
                        }
                    }
                    next.borrow_mut().data.vertex_type = next_vertex_type;
                }
                VertexType::Ear => {
                    if next.borrow().data.vertex_type == VertexType::Reflex {
                        match index {
                            Some(i) => {
                                reflexes.remove(i);
                            }
                            None => {}
                        }
                        next.borrow_mut().data.vertex_type = next_vertex_type;
                        ears.push(next);
                    } else if next.borrow().data.vertex_type == VertexType::Convex {
                        next.borrow_mut().data.vertex_type = next_vertex_type;
                        ears.push(next);
                    }
                }
                VertexType::Reflex => {
                    if next.borrow().data.vertex_type == VertexType::Ear {
                        match index {
                            Some(i) => {
                                ears.remove(i);
                            }
                            None => {}
                        }
                        next.borrow_mut().data.vertex_type = next_vertex_type;
                        reflexes.push(next);
                    } else if next.borrow().data.vertex_type == VertexType::Convex {
                        next.borrow_mut().data.vertex_type = next_vertex_type;
                        reflexes.push(next);
                    }
                }
            };
        }
    }
    if vertices.len() == 3 {
        let head = vertices.head().unwrap();
        let prev = head.borrow().prev.clone().unwrap();
        let next = head.borrow().next.clone().unwrap();
        let mut vec = vec![
            head.borrow().data.index,
            prev.borrow().data.index,
            next.borrow().data.index,
        ];
        vec.sort();
        triangles.push(vec);
    }
    triangles
}

fn get_vertex_type<T: NumberType>(
    ears: &Vec<Rc<RefCell<ListNode<EarcutVertex<T>>>>>,
    reflexes: &Vec<Rc<RefCell<ListNode<EarcutVertex<T>>>>>,
    vertex: Rc<RefCell<ListNode<EarcutVertex<T>>>>,
) -> (VertexType, Option<usize>) {
    let cur = vertex.clone();
    let prev = cur.borrow().prev.clone().unwrap();
    let next = cur.borrow().next.clone().unwrap();
    let mut points = vec![prev.borrow().data, cur.borrow().data, next.borrow().data];
    points.sort_by(|a, b| a.index.cmp(&b.index));
    let triangle = Triangle2::new(points[0].point, points[1].point, points[2].point);

    match triangle.orientation() {
        Orientation::Clockwise => match vertex.borrow().data.vertex_type {
            VertexType::Reflex => (VertexType::Reflex, None),
            VertexType::Ear => {
                let ear_index = ears
                    .iter()
                    .position(|x| x.borrow().data.point == cur.borrow().data.point);
                (VertexType::Reflex, ear_index)
            }
            VertexType::Convex => (VertexType::Reflex, None),
        },
        Orientation::CounterClockwise => {
            for i in 0..reflexes.len() {
                let reflex = reflexes[i].clone();
                if reflex.borrow().data.point == cur.borrow().data.point {
                    continue;
                }
                let point = reflex.borrow().data.point;
                let location = locate_point_2_triangle_2(&point, &triangle);
                match location {
                    Point2Triangle2Location::Inside => {
                        return (VertexType::Convex, None);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            match vertex.borrow().data.vertex_type {
                VertexType::Reflex => {
                    let reflex_index = reflexes
                        .iter()
                        .position(|x| x.borrow().data.point == cur.borrow().data.point);
                    (VertexType::Ear, reflex_index)
                }
                VertexType::Ear => (VertexType::Ear, None),
                VertexType::Convex => (VertexType::Ear, None),
            }
        }
    }
}

fn init_vertex_type<T: NumberType>(vertex: &Vec<Point2<T>>, index: usize) -> VertexType {
    if index >= vertex.len() {
        panic!("Index out of bounds");
    }
    let cur = vertex[index];
    let prev = vertex[(index + vertex.len() - 1) % vertex.len()];
    let next = vertex[(index + 1) % vertex.len()];
    let triangle = Triangle2::new(prev, cur, next);
    match triangle.orientation() {
        Orientation::Clockwise => {
            return VertexType::Reflex;
        }
        Orientation::CounterClockwise => {
            for i in 0..vertex.len() {
                if i == index
                    || i == (index + 1) % vertex.len()
                    || i == (index + vertex.len() - 1) % vertex.len()
                {
                    continue;
                }
                let point = &vertex[i];
                let location = locate_point_2_triangle_2(point, &triangle);
                match location {
                    Point2Triangle2Location::Inside => {
                        return VertexType::Convex;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            return VertexType::Ear;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earcut_2() {
        // let polygon = Polygon2::new(vec![
        //     Point2::new(0.0, 0.0),
        //     Point2::new(1.0, 0.0),
        //     Point2::new(1.0, 1.0),
        //     Point2::new(0.0, 1.0),
        // ]);
        // let triangles = earcut_2(polygon);
        // println!("{:?}", triangles);

        let polygon = Polygon2::new(vec![
            Point2::new(-5.0, -10.0),
            Point2::new(0.0, -15.0),
            Point2::new(10.0, -13.0),
            Point2::new(10.0, -5.0),
            Point2::new(2.0, -2.0),
            Point2::new(8.0, 8.0),
            Point2::new(4.0, 10.0),
            Point2::new(3.0, 3.0),
            Point2::new(0.0, 10.0),
            Point2::new(-2.0, 12.0),
        ]);
        let triangles = earcut_2(polygon);
        println!("{:?}", triangles);
    }
}
