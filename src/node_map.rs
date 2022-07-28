use rand::Rng;

use crate::{node::Node, rectangle::Rectangle, vert::Vertex, HEIGHT, NOISE, SIZE, WIDTH};

pub struct NodeMap {
    pub nodes: Vec<Vec<Node>>,
}

type Coordinate = (usize, usize);

fn usize_shift(original: usize, shift: i32) -> Option<usize> {
    return i32::try_from(original).ok().and_then(|i32_original| {
        let shifted = i32_original + shift;

        if shifted < 0 {
            return None;
        }

        return usize::try_from(shifted).ok();
    });
}

impl Default for NodeMap {
    fn default() -> Self {
        Self {
            nodes: vec![vec![]],
        }
    }
}

impl NodeMap {
    pub fn for_screen() -> Self {
        let mut node_map = NodeMap::default();

        let mut y = 0.0;
        while y < HEIGHT {
            let mut x = 0.0;
            while x < WIDTH {
                let top_left = Vertex {
                    position: [x as f32, y as f32],
                };

                let bottom_right = Vertex {
                    position: [(x + SIZE) as f32, (y + SIZE) as f32],
                };

                let rectangle = Rectangle {
                    top_left,
                    bottom_right,
                };

                let node = Node::with_rec(rectangle);

                node_map.add_node(node);

                x += SIZE;
            }

            node_map.add_row();
            y += SIZE;
        }

        return node_map;
    }
    pub fn add_row(&mut self) {
        self.nodes.push(vec![]);
    }

    fn get_at_mut(&mut self, coordinate: Coordinate) -> Option<&mut Node> {
        return self
            .nodes
            .get_mut(coordinate.1)
            .and_then(|row| return row.get_mut(coordinate.0));
    }

    fn get_at(&self, coordinate: Coordinate) -> Option<&Node> {
        return self
            .nodes
            .get(coordinate.1)
            .and_then(|row| return row.get(coordinate.0));
    }

    pub fn neighbors(&self, coordinate: Coordinate) -> Vec<&Node> {
        let x = coordinate.0;
        let y = coordinate.1;

        let relative_coordinates = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        return relative_coordinates
            .iter()
            .filter_map(|cor| {
                return usize_shift(x, cor.0).and_then(|usize_x| {
                    return usize_shift(y, cor.1).and_then(|usize_y| Some((usize_x, usize_y)));
                });
            })
            .filter_map(|coordinate| self.get_at(coordinate))
            .collect();
    }

    pub fn alive_neighbors(&self, coordinate: Coordinate) -> Vec<&Node> {
        return self
            .neighbors(coordinate)
            .into_iter()
            .filter(|node| node.is_on)
            .collect();
    }

    pub fn calculate(&mut self) {
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes.get(y).unwrap().len() {
                let alive_count = self.alive_neighbors((x, y)).len();
                let node = self.get_at_mut((x, y)).unwrap();

                if node.is_on && (alive_count == 2 || alive_count == 3) {
                    node.will_be_on = true;
                    continue;
                }

                if !node.is_on && alive_count == 3 {
                    node.will_be_on = true;
                    continue;
                }

                if NOISE {
                    let random = rand::thread_rng().gen_range(0..1000);
                    if random < 1 {
                        node.will_be_on = true;
                        continue;
                    }
                }

                node.will_be_on = false;
            }
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let last_option = self.nodes.last_mut();

        if let Some(row) = last_option {
            row.push(node);
        }
    }
}
