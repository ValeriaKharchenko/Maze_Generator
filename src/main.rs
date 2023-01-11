use crate::maze::{Grid, HIGH, LOW, MEDIUM};
use macroquad::prelude::*;

mod maze;


fn window_conf() -> Conf {
    Conf {
        window_title: "MAZE".to_owned(),
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid = Grid::new(10, 10, MEDIUM);
    grid.generate_maze();
    loop {
        grid.draw_maze();
        next_frame().await
    }
}