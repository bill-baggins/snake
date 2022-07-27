use std::collections::VecDeque;

use macroquad::rand;

use crate::globals::global::*;

pub struct Snake {
    pub body: VecDeque<Point>,
}

impl Snake {
    pub fn new() -> Self {
        let body = VecDeque::from([
            (rand::gen_range(0, BOARD_X) as i32, rand::gen_range(0, BOARD_Y) as i32),
        ]);

        Snake { body }
    }
}