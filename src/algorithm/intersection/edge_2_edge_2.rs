use crate::{
    algorithm::intersection::segment_2_segment_2::segment_2_segment_2_intersection,
    kernel::{edge_2::Edge2, number_type::NumberType, util_enum::Edge2Type, vertex_2::Vertex2},
};

pub fn edge_2_edge_2_intersection<T: NumberType>(e1: &Edge2<T>, e2: &Edge2<T>) -> Vec<Vertex2<T>> {
    let edge_type_1 = e1.edge_type();
    let edge_type_2 = e2.edge_type();
    if edge_type_1 == edge_type_2 {
        match edge_type_1 {
            Edge2Type::Segment => {
                let s1 = e1.to_segment();
                let s2 = e2.to_segment();
                let result = segment_2_segment_2_intersection(&s1, &s2);
                return result
                    .iter()
                    .map(|point| Vertex2::new(point.x(), point.y()))
                    .collect();
            }
            Edge2Type::Arc => {
                panic!("Not implemented yet")
            }
        }
    }
    panic!("Not implemented yet")
}
