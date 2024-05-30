use crate::{
    algorithm::intersection::segment_2_segment_2::segment_2_segment_2_intersection,
    kernel::{
        base_dcel::{
            base_edge_2::{BaseEdge2, BaseEdge2Type},
            base_vertex_2::BaseVertex2,
        },
        base_kernel::base_point_2::BasePoint2,
    },
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

pub fn edge_2_edge_2_intersection<'a, NT, T>(e1: &'a T, e2: &'a T) -> Vec<T::Vertex>
where
    NT: BaseNumberTypeTrait,
    T: BaseEdge2<'a, NT>,
{
    let edge_type_1 = e1.edge_type();
    let edge_type_2 = e2.edge_type();
    if edge_type_1 == edge_type_2 {
        match edge_type_1 {
            BaseEdge2Type::Segment => {
                let s1 = e1.to_segment();
                let s2 = e2.to_segment();
                let result = segment_2_segment_2_intersection(&s1, &s2);
                return result
                    .iter()
                    .map(|point| T::Vertex::new(point.x(), point.y()))
                    .collect();
            }
            BaseEdge2Type::Arc => {
                panic!("Not implemented yet")
            }
        }
    }
    panic!("Not implemented yet")
}
