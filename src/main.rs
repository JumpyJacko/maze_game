use console::{
    Key::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Escape},
    Term,
};
use clap::{Arg, Command};
use std::time::Instant;

use crate::{graph::{Graph, Path}, state::State};

mod entity;
mod graph;
mod state;

pub const SIZEY: usize = 21; // with padding
pub const SIZEX: usize = 21; // with padding

pub type Point = (usize, usize);
pub type Maze = [[usize; SIZEX]; SIZEY];

pub const TILESET: [&str; 6] = [
    "..",
    "##",
    "\x1b[34m@\x1b[0m.",
    "\x1b[42m..\x1b[0m",
    /* Path Finding Tiles, 4: Explored, 5: Solved Path */
    "\x1b[45m..\x1b[0m",
    "\x1b[41m..\x1b[0m",
];

fn main() {
    let matches = Command::new("maze_game")
        .version("0.3.0")
        .author("Jackson Ly (JumpyJacko)")
        .about("A small maze game with a pathfinding visualisation")
        .arg(Arg::new("path")
            .short('p')
            .long("pathfind")
            .default_value("")
            .help("Enables a pathfinding algorithm,\nchoose from (bfs, djikstra, a*)"))
        .get_matches();

    let path_alg: &str = matches.get_one::<String>("path").unwrap();

    
    let adj_neighbours: Graph = Graph::new(SIZEX, SIZEY);

    let grid = adj_neighbours.return_grid();
    let gen = grid.dfs_maze(0);
    let mut state = gen.to_state();

    let path: Path;

    let pathfind_timer = Instant::now();
    match path_alg {
        "bfs" => path = gen.bfs(0, ((SIZEY - 1) / 2) * ((SIZEX - 1) / 2) - 1 ),
        "djikstra" => todo!(),
        "a*" => todo!(),
        _ => path = Path::new(),
    }
    let pathfind_completion = pathfind_timer.elapsed().as_micros();

    let solved_state = path.plot_path(&state);

    print!("\x1B[2J\x1B[1;1H");
    state.render();
    let timer = Instant::now();
    
    let stdout = Term::buffered_stdout();
    
    print!("\x1B[{};1H", SIZEY + 3);
    if !path_alg.is_empty() {
        println!();
        solved_state.render();
        println!("Algorithm Compute Time: {} μs", pathfind_completion);
        println!("Key: \x1b[45m..\x1b[0m = Explored,   \x1b[41m..\x1b[0m = Shortest Path");
    }

    'game_loop: loop {
        if let Ok(key) = stdout.read_key() {
            print!("\x1B[1;1H");
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
