#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

