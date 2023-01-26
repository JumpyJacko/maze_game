type Point = (u32, u32);
type Maze = [[u32; 8]; 8];

#[derive(Debug)]
struct Entity {
    name: String,
    position: Point,
}

#[derive(Debug)]
struct State {
    maze: Maze,
    player: Entity,
    // If wanted, can add an enemy for theseus and the minotaur
}

impl Entity {
    fn new(name: String, position: Point) -> Entity {
        Entity {
            name,
            position
        }
    }

    fn move_entity(self, position: Point) -> Entity {
        // Needs to check for valid position
        // i.e. Wall, whether new position is adjacent to current position
        // Maybe opt for a direction and match statement
        // Consider adding an enum name Direction
        todo!();
    }
}

impl State {
    fn new(maze: Maze, spawn: Point) -> State {
        State {
            maze,
            player: Entity::new("Player".to_owned(), spawn),
        }
    }

    fn update(self, /*positions: Some(Vec)*/) -> State {
        todo!();
    }

    fn render(self) {
        /*
        Tileset will probably be,
           0 = .  | 1 = ## | 2 = @  | 3 = M  
           - read from State/self
           - use numbers in State/self to index the tileset
           - render & profit!
         */
        todo!();
    }

    fn check_win_state(self) -> bool {
        todo!();
    }
}

fn main() {
    // TODO: Procedurally generate a maze using the Depth-First Search (DFS) method to make 
    //       an adjacent neighbours table to then translate into a 2D array like this one
    let maze: [[u32; 8]; 8] = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let state = State::new(maze, (1, 1));

    println!("{:?}", state);
}
