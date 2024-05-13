use super::{face_2::Face2, vertex_2::Vertex2};

pub struct Edge2 {
    pub origin: Box<Vertex2>,
    pub destination: Box<Vertex2>,
    pub twin: Option<Box<Edge2>>,
    pub next: Option<Box<Edge2>>,
    pub prev: Option<Box<Edge2>>,
    pub face: Option<Box<Face2>>,
}
