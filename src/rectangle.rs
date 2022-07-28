use crate::triangle::{Triangle, buffer_from_triangle};
use crate::vert::Vertex;

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub top_left: Vertex,
    pub bottom_right: Vertex,
}

impl Rectangle {
    pub fn to_triangles(&self) -> [Triangle; 2] {
        let top_right = Vertex {
            position: [self.bottom_right.position[0], self.top_left.position[1]],
        };

        let bottom_left = Vertex {
            position: [self.top_left.position[0], self.bottom_right.position[1]],
        };

        return [
            Triangle {
                a: self.top_left,
                b: top_right,
                c: bottom_left,
            },
            Triangle {
                a: top_right,
                b: self.bottom_right,
                c: bottom_left,
            },
        ];
    }
}

pub fn buffers_from_rec(
    rectangle: Rectangle,
    display: &glium::Display,
) -> [glium::VertexBuffer<Vertex>; 2] {
    let triangles = rectangle.to_triangles();

    return [
        buffer_from_triangle(triangles[0], display),
        buffer_from_triangle(triangles[1], display),
    ];
}
