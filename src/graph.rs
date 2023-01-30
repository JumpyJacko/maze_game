use crate::*;

#[derive(Debug)]
pub struct Graph(Vec<Vec<usize>>);

impl Graph {
    pub fn new(size_x: usize, size_y: usize) -> Graph {
        Graph(vec![Vec::new(); ((size_y - 1) / 2) * ((size_x - 1) / 2)])
    }

    pub fn add_edge(&mut self, node_1: usize, node_2: usize) -> Graph {
        self.0[node_1].push(node_2);
        self.0[node_2].push(node_1);
        Graph(self.0.clone())
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
            if point.0 > SIZEX {        // HOW IS THIS WORKING????????
                grid.add_edge(head.0, head.0 + 1);
            }
            if point.1 > SIZEY {        // HOW IS THIS WORKING????????
                grid.add_edge(head.0, head.0 + ((SIZEY - 1) / 2));
            }
        });

        grid
    }

    pub fn depth_first_search(&self) -> Graph {
        // First, create a grid adjacency list (graph), probably put in 'fn new'
        // do dfs
        todo!();
    }

    /// For debugging purposes
    // pub fn print_graph(&self) {
    //     self.0.iter().enumerate().for_each(|node| {
    //         print!("\nAdjacency list of node: {}\nhead", node.0);
    //         node.1.iter().for_each(|node| {
    //             print!(" -> {}", node);
    //         });
    //     });
    // }

    pub fn to_state(&self) -> State {
        let mut maze: Maze = [[1; SIZEX]; SIZEY]; // kinda stupid

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
