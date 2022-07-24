pub mod globals;
pub mod snake;
pub mod game;
pub mod board;

use crate::globals::global::*;
use game::Game;
use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: TITLE.to_string(), 
        window_width: WIDTH, 
        window_height: HEIGHT, 
        fullscreen: FULLSCREEN, 
        window_resizable: false, 
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut game = Game::new();
    loop {
        clear_background(SKYBLUE);
        game.get_input();
        game.update();
        game.draw();

        next_frame().await;
    }
}
