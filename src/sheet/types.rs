use std::error::Error;

use crate::graph::types::{Graph, GraphNode, Id, ValueNode};
use super::parsing::Parser;
struct Coord<T> {
    pub x: T,
    pub y: T
}

pub struct Sheet<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Option<Id>; WIDTH]; HEIGHT],
    pub graph: Graph,
}

impl<const WIDTH: usize, const HEIGHT: usize> Sheet<{ WIDTH }, { HEIGHT }> {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            cells: [[None; WIDTH]; HEIGHT]
        }
    }

    pub fn add_node(&mut self, coords: Coord<usize>, node: Box<dyn CellPrint>) {
        self.cells[coords.y][coords.x] = Some(node.id());
        self.graph.add_node(node);
    }

    pub fn get_node(&self, coords: Coord<usize>) -> Option<Id> {
        return self.cells[coords.y][coords.x]
    }

    pub fn set_cell_parse(&mut self, raw: String) -> Result<Id, Box<dyn Error>> {
    }
}


pub trait CellPrint: GraphNode {
    fn print_cell(&self, graph: &Graph) -> String;
}

impl CellPrint for ValueNode {
    fn print_cell(&self, graph: &Graph) -> String {
        self.value(&graph).to_string()
    }
}