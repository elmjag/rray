use rray::{math::is_close, scene::Vertex};

pub fn vertices_are_close(v1: Vertex, v2: Vertex) -> bool {
    is_close(v1.x(), v2.x()) && is_close(v1.y(), v2.y()) && is_close(v1.z(), v2.z())
}
