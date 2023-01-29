use console::{
    Key::{ArrowLeft, ArrowRight, ArrowUp, ArrowDown, Escape},
    Term
};

use crate::{state::State, graph::Graph};

mod entity;
mod state;
mod graph;

pub const SIZEY: usize = 9; // with padding
pub const SIZEX: usize = 9; // with padding

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

    let mut adj_neighbours: Graph = Graph::new();

    adj_neighbours.add_edge(0, 1);
    adj_neighbours.add_edge(1, 2);
    adj_neighbours.add_edge(2, 6);
    adj_neighbours.add_edge(5, 6);
    adj_neighbours.add_edge(5, 4);
    adj_neighbours.add_edge(5, 9);
    adj_neighbours.add_edge(8, 9);
    adj_neighbours.add_edge(13, 9);
    adj_neighbours.add_edge(13, 12);
    adj_neighbours.add_edge(13, 14);
    adj_neighbours.add_edge(10, 14);
    adj_neighbours.add_edge(10, 11);
    adj_neighbours.add_edge(15, 11);
    adj_neighbours.add_edge(7, 11);
    adj_neighbours.add_edge(7, 3);

    // let mut state = State::new(maze, (1, 1), (5, 1));
    let mut state = adj_neighbours.to_state();

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
