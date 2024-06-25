use std::collections::HashSet;

use crate::{
    algorithm::location::{
        location_enum::Point2Triangle2Location, point_2_triangle_2::locate_point_2_triangle_2,
    },
    kernel::{
        number_type::NumberType, point_2::Point2, polygon_2::Polygon2, triangle_2::Triangle2,
        util_enum::Orientation,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VertexType {
    Convex,
    Reflex,
    Ear,
}

#[derive(Clone, Copy)]
struct EarcutVertex<T: NumberType> {
    index: usize,
    point: Point2<T>,
    vertex_type: VertexType,
}

pub fn earcut_2<T: NumberType>(polygon: Polygon2<T>) -> Vec<Vec<usize>> {
    let is_simple = polygon.is_simple();
    if !is_simple {
        panic!("Polygon is not simple");
    }
    let mut vertices = polygon.vertices();
    let mut ears = Vec::new();
    let mut convexes = Vec::new();
    let mut reflexes = Vec::new();
    for i in 0..vertices.len() {
        let vertex_type = get_vertex_type(&vertices, i);
        match vertex_type {
            VertexType::Convex => {
                convexes.push(EarcutVertex {
                    index: i,
                    point: vertices[i],
                    vertex_type,
                });
            }
            VertexType::Reflex => {
                reflexes.push(EarcutVertex {
                    index: i,
                    point: vertices[i],
                    vertex_type,
                });
            }
            VertexType::Ear => {
                ears.push(EarcutVertex {
                    index: i,
                    point: vertices[i],
                    vertex_type,
                });
            }
        }
    }
    todo!()
}

fn get_vertex_type<T: NumberType>(points: &Vec<Point2<T>>, index: usize) -> VertexType {
    if index >= points.len() {
        panic!("Index out of bounds");
    }
    let cur = points[index];
    let prev = points[(index + points.len() - 1) % points.len()];
    let next = points[(index + 1) % points.len()];
    let triangle = Triangle2::new(prev, cur, next);
    match triangle.orientation() {
        Orientation::Clockwise => {
            return VertexType::Reflex;
        }
        Orientation::CounterClockwise => {
            for i in 0..points.len() {
                if i == index
                    || i == (index + 1) % points.len()
                    || i == (index + points.len() - 1) % points.len()
                {
                    continue;
                }
                let point = &points[i];
                let location = locate_point_2_triangle_2(point, &triangle);
                match location {
                    Point2Triangle2Location::Inside => {
                        continue;
                    }
                    _ => {
                        return VertexType::Convex;
                    }
                }
            }
            return VertexType::Ear;
        }
    }
}
