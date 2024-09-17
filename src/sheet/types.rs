use crate::graph::types::{Graph, GraphNode, Id, ValueNode};
use std::error::Error;

pub struct Coord<T> {
    pub x: T,
    pub y: T,
}


#[derive(PartialEq)]
pub struct Sheet<const WIDTH: usize, const HEIGHT: usize> {
    pub cells: [[Option<Id>; WIDTH]; HEIGHT],
    pub graph: Graph,
}

impl<const WIDTH: usize, const HEIGHT: usize> Sheet<{ WIDTH }, { HEIGHT }> {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    pub fn add_node(&mut self, coords: Coord<usize>, node: Box<dyn CellPrint>) -> Id {
        let id = node.id();
        self.cells[coords.y][coords.x] = Some(id);
        self.graph.add_node(node);
        id
    }

    pub fn get_node(&self, coords: Coord<usize>) -> Option<Id> {
        return self.cells[coords.y][coords.x];
    }

    pub fn set_cell_parse(&mut self, raw: String, coords: Coord<usize>) -> Result<Id, Box<dyn Error>> {
        let id = self.parse_to_cell(raw)?;
        self.cells[coords.y][coords.x] = Some(id);

        Ok(id)
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
