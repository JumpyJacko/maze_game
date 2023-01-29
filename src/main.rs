use console::{
    Key::{ArrowLeft, ArrowRight, ArrowUp, ArrowDown, Escape},
    Term
};

use crate::state::State;

mod entity;
mod state;

pub const SIZEY: usize = 8;
pub const SIZEX: usize = 8;

pub type Point = (usize, usize);
pub type Maze = [[usize; SIZEX]; SIZEY];

pub const TILES: [&str; 4] = ["..", "##", "\x1b[34m@\x1b[0m.", "\x1b[42m..\x1b[0m"];

fn main() {
    // TODO: Procedurally generate a maze using the Depth-First Search (DFS) method to make
    //       an adjacent neighbours table to then translate into a 2D array like this one
    // NOTE: To get he size of the DFS adjacent neighbours table, use (SIZEX/Y - 1) / 2
    //       Will have to create walls
    let maze: Maze = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut state = State::new(maze, (1, 1), (5, 1));

    print!("\x1B[2J\x1B[1;1H");
    state.render();

    let stdout = Term::buffered_stdout();
    'game_loop: loop {
        print!("\x1B[1;1H");
        if let Ok(key) = stdout.read_key() {
            match key {
                ArrowLeft  => state = state.input(ArrowLeft),
                ArrowRight => state = state.input(ArrowRight),
                ArrowUp    => state = state.input(ArrowUp),
                ArrowDown  => state = state.input(ArrowDown),
                Escape     => break 'game_loop,
                _ => break 'game_loop, // FIXME: Handle later
            };
            state.update();
            state.render();
            if state.check_win_state() {
                println!("solved");
                break 'game_loop;
            };
        }
    }
}
