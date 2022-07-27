use macroquad::prelude::*;
use crate::globals::global::*;

type Board = [[State; BOARD_X]; BOARD_Y];

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut board = [[State::EMPTY; BOARD_X]; BOARD_Y];
        Game { board }
    }

    pub fn get_input(&mut self) {
        
    }

    pub fn update(&mut self) {
    
    }

    pub fn draw(&self) {
        self.draw_board();
    }

    fn draw_board(&self) {
        for y in 0..BOARD_Y {
            for x in 0..BOARD_X {
                let cell = self.board[y][x];
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