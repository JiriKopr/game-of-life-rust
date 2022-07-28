use crate::vert::Vertex;
use crate::{HEIGHT, WIDTH};

pub fn transform_from_pixels(vertex: Vertex) -> Vertex {
    return Vertex {
        position: [
            vertex.position[0] / WIDTH * 2.0 - 1.0,
            vertex.position[1] / HEIGHT * -2.0 + 1.0,
        ],
    };
}
