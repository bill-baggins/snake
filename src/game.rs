use macroquad::prelude::*;
use crate::globals::global::*;

use crate::snake::Snake;
use crate::board::Board;


pub struct Game {
    snake: Snake,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let snake = Snake::new();
        let mut board = Board::new();
        board.set(4, 4, State::FRUIT);
        Game { snake, board }
    }

    pub fn get_input(&mut self) {

    }

    pub fn update(&mut self) {
        if (self.snake.)
    }

    pub fn draw(&self) {
        self.board.draw()
    }
}