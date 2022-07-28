use crate::{vert::Vertex, utils::transform_from_pixels};

#[derive(Copy, Clone)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

pub fn buffer_from_triangle(
    triangle: Triangle,
    display: &glium::Display,
) -> glium::VertexBuffer<Vertex> {
    let shapes = vec![triangle.a, triangle.b, triangle.c]
        .iter()
        .map(|&x| transform_from_pixels(x))
        .collect::<Vec<Vertex>>();

    return glium::VertexBuffer::new(display, &shapes).unwrap();
}
