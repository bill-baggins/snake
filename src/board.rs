use crate::globals::global::*;
use macroquad::prelude::*;

pub struct Board {
    arr: [[State; BOARD_X]; BOARD_Y]
}

impl Board {
    pub fn new() -> Self {
        let arr = [[State::EMPTY; BOARD_X]; BOARD_Y];

        Board { arr }
    }

    pub fn get(&self, x: usize, y: usize) -> State {
        self.arr[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: State) {
        self.arr[y][x] = val;
    }

    pub fn draw(&self) {
        for y in 0..BOARD_Y {
            for x in 0..BOARD_X {
                let cell = self.arr[y][x];
                let color = match cell {
                    State::EMPTY => GRAY,
                    State::FRUIT => RED,
                    State::SNAKE => GREEN
                };
                draw_rectangle((x as i32 * CELL_X) as f32, (y as i32 * CELL_Y) as f32, CELL_X as f32, CELL_Y as f32, color);
            }
        }
    }
}