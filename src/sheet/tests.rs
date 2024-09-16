use crate::graph::types::{Graph, ValueNode};
use super::types::{*};

#[test]
fn print_valuenode() {
    let graph = Graph::new();

    let node = ValueNode::new(5.0);
    assert_eq!(node.print_cell(&graph), "5".to_owned());

    let node2 = ValueNode::new(3.1415);
    assert_eq!(node2.print_cell(&graph), "3.1415".to_owned());
}