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

pub const TILES: [&str; 4] = ["..", "##", "\x1b[34m@\x1b[0m.", "\x1b[42m..\x1b[0m"];

fn main() {
    // TODO: Procedurally generate a maze using the Depth-First Search (DFS) method to make
    //       an adjacent neighbours table to then translate into a 2D array like this one
    // NOTE: To get he size of the DFS adjacent neighbours table, use (SIZEX/Y - 1) / 2
    //       Will have to create walls
    // let maze: Maze = [
    //     [1, 1, 1, 1, 1, 1, 1, 1, 1],
    //     [1, 0, 1, 0, 1, 0, 0, 0, 1],
    //     [1, 0, 1, 0, 1, 1, 1, 0, 1],
    //     [1, 0, 0, 0, 1, 0, 1, 0, 1],
    //     [1, 0, 1, 0, 1, 0, 1, 0, 1],
    //     [1, 0, 1, 0, 0, 0, 0, 0, 1],
    //     [1, 0, 1, 0, 1, 0, 1, 0, 1],
    //     [1, 0, 1, 0, 0, 0, 1, 0, 1],
    //     [1, 1, 1, 1, 1, 1, 1, 1, 1],
    // ];

    let adj_neighbours: Graph = Graph::new(SIZEX, SIZEY);

    // adj_neighbours.add_edge(0, 1);
    // adj_neighbours.add_edge(1, 2);
    // adj_neighbours.add_edge(2, 6);
    // adj_neighbours.add_edge(5, 6);
    // adj_neighbours.add_edge(5, 4);
    // adj_neighbours.add_edge(5, 9);
    // adj_neighbours.add_edge(8, 9);
    // adj_neighbours.add_edge(13, 9);
    // adj_neighbours.add_edge(13, 12);
    // adj_neighbours.add_edge(13, 14);
    // adj_neighbours.add_edge(10, 14);
    // adj_neighbours.add_edge(10, 11);
    // adj_neighbours.add_edge(15, 11);
    // adj_neighbours.add_edge(7, 11);
    // adj_neighbours.add_edge(7, 3);

    // let mut state = State::new(maze, (1, 1), (5, 1));
    let grid = adj_neighbours.return_grid();
    let gen = grid.dfs_maze(0);
    let mut state = gen.to_state();
    println!();

    print!("\x1B[2J\x1B[1;1H");
    state.render();
    let timer = Instant::now();

    let stdout = Term::buffered_stdout();
    'game_loop: loop {
        print!("\x1B[1;1H");
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
