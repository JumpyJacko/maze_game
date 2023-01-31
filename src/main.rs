use console::{
    Key::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Escape},
    Term,
};
use std::time::Instant;

use crate::{graph::Graph, state::State};

mod entity;
mod graph;
mod state;

pub const SIZEY: usize = 29; // with padding
pub const SIZEX: usize = 29; // with padding

pub type Point = (usize, usize);
pub type Maze = [[usize; SIZEX]; SIZEY];

pub const TILESET: [&str; 7] =
    ["..", "##", "\x1b[34m@\x1b[0m.", "\x1b[42m..\x1b[0m",
    /* Path Finding Tiles, 4: Explored, 5: Explored Head/End, 6: Solved Path */
    "\x1b[45m..\x1b[0m", "\x1b[105m..\x1b[0m", "\x1b[41m..\x1b[0m"];

fn main() {
    // TODO: Really need to make a cli to choose between playing or pathfinding, probably just
    //       use clap again because its really nice and easy.
    let adj_neighbours: Graph = Graph::new(SIZEX, SIZEY);

    let grid = adj_neighbours.return_grid();
    let gen = grid.dfs_maze(0);
    let mut state = gen.to_state();
    println!();

    print!("\x1B[2J\x1B[1;1H");
    state.render();
    let timer = Instant::now();

    let stdout = Term::buffered_stdout();
    'game_loop: loop {
        if let Ok(key) = stdout.read_key() {
            match key {
                ArrowLeft => state = state.input(ArrowLeft),
                ArrowRight => state = state.input(ArrowRight),
                ArrowUp => state = state.input(ArrowUp),
                ArrowDown => state = state.input(ArrowDown),
                Escape => break 'game_loop,
                _ => break 'game_loop, // FIXME: Handle later
            };
            state.update();
            state.render();
            if state.check_win_state() {
                break 'game_loop;
            };
        }
    }
    let completion_time = timer.elapsed();
    let secs = completion_time.as_secs();
    let mins = secs / 60;
    let millis = (completion_time.subsec_millis()) as u64;
    println!("Completion Time: {}:{}.{}", mins, secs, millis);
}
