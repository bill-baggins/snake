
pub mod global {
    pub const WIDTH: i32 = 800;
    pub const HEIGHT: i32 = 600;
    pub const FPS: i32 = 75;
    pub const FULLSCREEN: bool = false;
    pub const TITLE: &str = "Snake";
    
    pub const CELL_X: i32 = 40;
    pub const CELL_Y: i32 = 40;

    pub const BOARD_X: usize = WIDTH as usize / CELL_X as usize;
    pub const BOARD_Y: usize = HEIGHT as usize / CELL_Y as usize;

    #[derive(Debug, Clone, Copy)]
    pub enum State {
        EMPTY,
        SNAKE,
        FRUIT
    }

    pub type Point = (i32, i32);
}

