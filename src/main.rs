use console::{Key, Term};

const SIZEY: usize = 8;
const SIZEX: usize = 8;

type Point = (usize, usize);
type Maze = [[usize; SIZEX]; SIZEY];

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug, Copy, Clone)]
struct Entity {
    name: &'static str,
    position: Point,
}

#[derive(Debug)]
struct State {
    maze: Maze,
    finish: Point,
    player: Entity,
    // If wanted, can add an enemy for theseus and the minotaur
}

const TILES: [&str; 4] = ["..", "##", "\x1b[34m@\x1b[0m.", "\x1b[42m..\x1b[0m"];

impl Entity {
    fn new(name: &'static str, position: Point) -> Entity {
        Entity { name, position }
    }

    fn move_entity(&self, direction: Direction) -> Entity {
        match direction {
            Direction::Right => Entity {
                name: self.name,
                position: (self.position.0 + 1, self.position.1),
            },
            Direction::Left => Entity {
                name: self.name,
                position: (self.position.0 - 1, self.position.1),
            },
            Direction::Down => Entity {
                name: self.name,
                position: (self.position.0, self.position.1 + 1),
            },
            Direction::Up => Entity {
                name: self.name,
                position: (self.position.0, self.position.1 - 1),
            },
        }
    }
}

impl State {
    fn new(mut maze: Maze, spawn: Point, finish: Point) -> State {
        maze[spawn.1][spawn.0] = 2;
        maze[finish.1][finish.0] = 3;

        State {
            maze,
            finish,
            player: Entity::new("Player", spawn),
        }
    }

    fn input(&mut self, p_input: Direction) -> State {
        self.maze[self.player.position.1][self.player.position.0] = 0; // Clear tile
        match p_input {
            Direction::Right => {
                if self.check_valid(self.player.move_entity(Direction::Right)) {
                    self.player = self.player.move_entity(Direction::Right)
                }
            }
            Direction::Left => {
                if self.check_valid(self.player.move_entity(Direction::Left)) {
                    self.player = self.player.move_entity(Direction::Left)
                }
            }
            Direction::Down => {
                if self.check_valid(self.player.move_entity(Direction::Down)) {
                    self.player = self.player.move_entity(Direction::Down)
                }
            }
            Direction::Up => {
                if self.check_valid(self.player.move_entity(Direction::Up)) {
                    self.player = self.player.move_entity(Direction::Up)
                }
            }
        };

        State {
            maze: self.maze,
            finish: self.finish,
            player: self.player,
        }
    }

    fn update(&mut self) {
        self.maze[self.player.position.1][self.player.position.0] = 2;
    }

    fn render(&self) {
        // print!("\x1B[1;1H");
        self.maze.iter().for_each(|row| {
            row.iter().for_each(|c| {
                print!("{}", TILES[*c]);
            });
            println!();
        });
    }

    fn check_valid(&self, entity: Entity) -> bool {
        matches!(self.maze[entity.position.1][entity.position.0], 3 | 0)
    }

    fn check_win_state(&self) -> bool {
        self.player.position == self.finish
    }

    fn to_adj_neighbours(&self) -> Vec<Vec<usize>> {
        todo!();
    }

    fn from_adj_neighbours(nodes: Vec<Vec<usize>>) -> State {
        todo!();
    }
}

fn main() {
    // TODO: Procedurally generate a maze using the Depth-First Search (DFS) method to make
    //       an adjacent neighbours table to then translate into a 2D array like this one
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
                Key::ArrowLeft  => state = state.input(Direction::Left),
                Key::ArrowRight => state = state.input(Direction::Right),
                Key::ArrowUp    => state = state.input(Direction::Up),
                Key::ArrowDown  => state = state.input(Direction::Down),
                Key::Escape     => break 'game_loop,
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
