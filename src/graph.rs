use crate::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Graph(Vec<Vec<usize>>);

pub struct Path {
    explored: Vec<Point>,
    path: Vec<Point>
}

impl Graph {
    pub fn new(size_x: usize, size_y: usize) -> Graph {
        Graph(vec![Vec::new(); ((size_y - 1) / 2) * ((size_x - 1) / 2)])
    }

    pub fn add_edge(&mut self, node_1: usize, node_2: usize) -> Graph {
        self.0[node_1].push(node_2);
        self.0[node_2].push(node_1);
        Graph(self.0.clone())
    }
    
    /// For debugging purposes
    pub fn _print_graph(&self) {
        self.0.iter().enumerate().for_each(|node| {
            print!("\nAdjacency list of node: {}\nhead", node.0);
            node.1.iter().for_each(|node| {
                print!(" -> {}", node);
            });
        });
    }

    pub fn return_grid(&self) -> Graph {
        let mut grid = Graph::new(SIZEX, SIZEY);
        self.0.iter().enumerate().for_each(|head| {
            let point = index_to_cartesian((SIZEX, SIZEY), head.0);
            if point.0 != 0 {
                grid.add_edge(head.0, head.0 - 1);
            }
            if point.1 != 0 {
                grid.add_edge(head.0, head.0 - ((SIZEX - 1) / 2));
            }
            if point.0 > SIZEX {
                grid.add_edge(head.0, head.0 + 1);
            }
            if point.1 > SIZEY {
                grid.add_edge(head.0, head.0 + ((SIZEY - 1) / 2));
            }
        });

        grid
    }

    /// Use with grid so that the dfs can traverse every thing
    pub fn dfs_maze(&self, start: usize) -> Graph {
        let mut marked = vec![false; self.0.len()];
        let mut maze = Graph::new(SIZEX, SIZEY);
        let mut dfs_order = vec![];

        let mut stack = vec![start];
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let mut neighbour_nodes = vec![];
            for &neighbour in &self.0[node] {
                if !marked[neighbour] {
                    neighbour_nodes.push(neighbour);
                }
            }
            neighbour_nodes.shuffle(&mut thread_rng());
            stack.extend(neighbour_nodes.iter().copied());

            // ChatGPT #2 + some modification
            for n_node in neighbour_nodes {
                if !marked[n_node] {
                    marked[n_node] = true;
                    maze.add_edge(node, n_node);
                    dfs_order.push(n_node);
                }
            }
        }

        // dfs_order.iter().for_each(|i| println!("{:?}", index_to_cartesian((SIZEX, SIZEY), *i)));

        maze
    }

    pub fn to_state(&self) -> State {
        let mut maze: Maze = [[1; SIZEX]; SIZEY];

        self.0.iter().enumerate().for_each(|head| {
            let head_coords = index_to_cartesian((SIZEX, SIZEY), head.0);
            let head_x: usize = head_coords.0;
            let head_y: usize = head_coords.1;
            maze[(head_y * 2) + 1][(head_x * 2) + 1] = 0;
            head.1.iter().for_each(|neighbour| {
                let neighbour_coords = index_to_cartesian((SIZEX, SIZEY), *neighbour);
                let neighbour_x: usize = neighbour_coords.0;
                let neighbour_y: usize = neighbour_coords.1;
                let inbetween_x = head_x + neighbour_x;
                let inbetween_y = head_y + neighbour_y;
                maze[inbetween_y + 1][inbetween_x + 1] = 0;
            });
        });

        State::new(maze, (1, 1), (SIZEX - 2, SIZEY - 2))
    }

    pub fn bfs(&self) -> Path {
        todo!();
    }

    pub fn djikstra(&self) -> Path {
        todo!();
    }

    pub fn astar(&self) -> Path {
        todo!();
    }
}

impl Path {
    pub fn new(explored: Vec<Point>, path: Vec<Point>) -> Path {
        Path {
            explored,
            path
        }
    }

    pub fn plot_path(&self, mut maze: state::State) -> state::State {
        for &point in &self.explored {
            maze.maze[point.1][point.0] = 4;
        }
        for &point in &self.path {
            maze.maze[point.1][point.0] = 6;
        }
        maze
    }
}

/// Returns an (x, y) cartesian coordinate for a 1D index
/// Note: includes the padding and walls between the coordinates so coordinates like (0,0) are impossible
fn index_to_cartesian(size: Point, index: usize) -> (usize, usize) {
    let size_x = size.0;
    let size_y = size.1;
    (
        index % ((size_x - 1) / 2),
        ((-((index % ((size_x - 1) / 2)) as isize) + index as isize) / ((size_y - 1) / 2) as isize)
            .try_into()
            .unwrap(),
    )
}
