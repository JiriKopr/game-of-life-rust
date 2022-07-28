use crate::rectangle::Rectangle;
use rand::Rng;

pub struct Node {
    pub is_on: bool,
    pub will_be_on: bool,
    pub rectangle: Rectangle,
}

impl Node {
    pub fn with_rec(rectangle: Rectangle) -> Self {
        let random = rand::thread_rng().gen_range(0..100);

        Self {
            is_on: random < 60,
            will_be_on: false,
            rectangle,
        }
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
        self.will_be_on = false;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.will_be_on = false;
    }
}
