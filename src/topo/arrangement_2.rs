use super::{edge_2::Edge2, face_2::Face2, vertex_2::Vertex2};

pub struct Arrangement2 {
    pub vertices: Vec<Vertex2>,
    pub edges: Vec<Edge2>,
    pub faces: Vec<Face2>,
}
