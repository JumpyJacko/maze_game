use crate::*;

#[derive(Debug)]
pub struct Graph(Vec<Vec<usize>>);

impl Graph {
    pub fn new() -> Graph {
        Graph(vec![Vec::new(); 1 + (((SIZEY - 1) / 2) * ((SIZEX - 1) / 2))])
    }

    pub fn add_edge(&mut self, node_1: usize, node_2: usize) -> Graph {
        self.0[node_1].push(node_2);
        self.0[node_2].push(node_1);
        Graph(self.0.clone()) 
    }

    pub fn print_graph(&self) {
        self.0.iter().enumerate().for_each(|node| {
            print!("\nAdjacency list of node: {}\nhead", node.0);
            node.1.iter().for_each(|node| {
                print!(" -> {}", node);
            });
        });
    }
    
    pub fn to_state(&self) -> State {
        // Internally, there is no border of '1's
        // Important (graph) nodes are on even numbered coords (inc. 0)
        let mut maze: Maze = [[1; SIZEX]; SIZEY];

        self.0.iter().enumerate().for_each(|head| {
            let head_x: usize = head.0 % ((SIZEX - 1) / 2); // - 2 to remove padding
            let head_y: usize = ((-(head_x as isize) + head.0 as isize) / ((SIZEY - 1) / 2) as isize).try_into().unwrap();
            maze[(head_y * 2) + 1][(head_x * 2) + 1] = 0;
            // head.1.iter().for_each(|neighbour| {
            //     let neighbour_x: usize = neighbour % SIZEX;
            //     let neighbour_y: usize = (-(neighbour_x as isize) + neighbour.into()) / SIZEY.into();
            // });
        });

        State {
            maze,
            finish: (SIZEX, SIZEY),
            player: entity::Entity::new("Player", (1, 1))
        }
    }
}