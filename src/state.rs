use crate::*;

use entity::Entity;

#[derive(Debug)]
pub struct State {
    pub maze: Maze,
    pub finish: Point,
    pub player: Entity,
    // If wanted, can add an enemy for theseus and the minotaur
}

impl State {
    pub fn new(mut maze: Maze, spawn: Point, finish: Point) -> State {
        maze[spawn.1][spawn.0] = 2;
        maze[finish.1][finish.0] = 3;

        State {
            maze,
            finish,
            player: Entity::new("Player", spawn),
        }
    }

    pub fn input(&mut self, p_input: console::Key) -> State {
        self.maze[self.player.position.1][self.player.position.0] = 0; // Clear tile
        match p_input {
            ArrowRight => {
                if self.check_valid(self.player.move_entity(ArrowRight)) {
                    self.player = self.player.move_entity(ArrowRight)
                }
            }
            ArrowLeft => {
                if self.check_valid(self.player.move_entity(ArrowLeft)) {
                    self.player = self.player.move_entity(ArrowLeft)
                }
            }
            ArrowDown => {
                if self.check_valid(self.player.move_entity(ArrowDown)) {
                    self.player = self.player.move_entity(ArrowDown)
                }
            }
            ArrowUp => {
                if self.check_valid(self.player.move_entity(ArrowUp)) {
                    self.player = self.player.move_entity(ArrowUp)
                }
            }
            _ => (),
        };

        State {
            maze: self.maze,
            finish: self.finish,
            player: self.player,
        }
    }

    pub fn update(&mut self) {
        self.maze[self.player.position.1][self.player.position.0] = 2;
    }

    pub fn render(&self) {
        self.maze.iter().for_each(|row| {
            row.iter().for_each(|c| {
                print!("{}", TILESET[*c]);
            });
            println!();
        });
    }

    pub fn check_valid(&self, entity: Entity) -> bool {
        matches!(
            self.maze[entity.position.1][entity.position.0],
            3 | 4 | 5 | 0
        )
    }

    pub fn check_win_state(&self) -> bool {
        self.player.position == self.finish
    }
}
