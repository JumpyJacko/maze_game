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
    
    pub fn from_graph(&self) -> State {
        todo!();
    }
}