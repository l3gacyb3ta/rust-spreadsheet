use crate::graph::types::{Graph, GraphNode, SumNode, ValueNode};
use std::collections::HashMap;

#[test]
fn create_value_node() {
    let v_node = ValueNode::new(1.5);
    assert_eq!(
        v_node.value(&Graph {
            nodes: HashMap::new(),
            edges: vec![],
        }),
        1.5
    );
}

#[test]
fn create_sum_node() {}

#[test]
fn basic_add_graph() {
    let mut graph = Graph::new();
    let a = graph.add_node(Box::new(ValueNode::new(1.0)));
    let b = graph.add_node(Box::new(ValueNode::new(2.0)));

    let a_b = graph.add_node(Box::new(SumNode::new()));
    graph.connect(a, a_b);
    graph.connect(b, a_b);

    assert_eq!(graph.get_node_value(a_b), Some(3.0));
}

#[test]
fn multi_layer_add_graph() {
    let mut graph = Graph::new();
    let a = graph.add_node(Box::new(ValueNode::new(1.0)));
    let b = graph.add_node(Box::new(ValueNode::new(2.0)));

    let a_b = graph.add_node(Box::new(SumNode::new()));
    graph.connect(a, a_b);
    graph.connect(b, a_b);

    assert_eq!(graph.get_node_value(a_b), Some(3.0));

    let c = graph.add_node(Box::new(ValueNode::new(3.)));
    let c_ab = graph.add_node(Box::new(SumNode::new()));

    graph.connect(a_b, c_ab);
    graph.connect(c, c_ab);

    assert_eq!(graph.get_node_value(c_ab), Some(6.0));
}

#[test]
fn three_add_node() {
    let mut graph = Graph::new();
    let a = graph.add_node(Box::new(ValueNode::new(1.0)));
    let b = graph.add_node(Box::new(ValueNode::new(2.0)));
    let c = graph.add_node(Box::new(ValueNode::new(3.0)));

    let abc = graph.add_node(Box::new(SumNode::new()));
    graph.connect(a, abc);
    graph.connect(b, abc);
    graph.connect(c, abc);

    assert_eq!(graph.get_node_value(abc), Some(6.0));
}

#[test]
fn remove_node() {
    let mut graph = Graph::new();
    let a = graph.add_node(Box::new(ValueNode::new(1.0)));
    let b = graph.add_node(Box::new(SumNode::new()));
    graph.connect(a, b);
    let not_empty: Vec<u64> = vec![a];
    assert_eq!(graph.get_dependants(b), not_empty);

    graph.remove_node(b);

    let empty: Vec<u64> = vec![];
    assert_eq!(graph.get_dependants(b), empty);
}