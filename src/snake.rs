use macroquad::prelude::*;
use crate::globals::global::*;
use std::collections::VecDeque;

pub struct Snake {
    body: VecDeque<Point>,
}

impl Snake {
    pub fn new() -> Self {
        let body = VecDeque::from([
            (0, 0),
        ]);
        Snake { body }
    }

    pub fn get_input(&mut self) {
        let key = get_last_key_pressed().unwrap_or(KeyCode::Key0);
        match key {
            KeyCode::W | KeyCode::Up => {
                // Logic for moving up.
            },
            KeyCode::A | KeyCode::Left => {
                // Logic for moving left.
            },
            KeyCode::S | KeyCode::Down => {
                // Logic for moving down.
            },
            KeyCode::D | KeyCode::Right => {
                // Logiv for moving right.
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {

    }
}