use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::atomic::AtomicU64;

pub type Id = u64;
static COUNTER: AtomicU64 = AtomicU64::new(0);

fn generateId() -> Id {
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub trait GraphNode: std::fmt::Debug {
    fn id(&self) -> Id;
    fn value(&self, graph: &Graph) -> f32;
}

#[derive(Debug)]
pub struct ValueNode {
    id: Id,
    value: f32,
}

impl ValueNode {
    pub fn new(value: f32) -> Self {
        return Self {
            id: generateId(),
            value,
        };
    }
}

impl GraphNode for ValueNode {
    fn id(&self) -> Id {
        self.id
    }

    fn value(&self, _: &Graph) -> f32 {
        self.value
    }
}

#[derive(Debug)]
pub struct SumNode {
    id: Id,
}

impl SumNode {
    pub fn new() -> Self {
        let node = Self { id: generateId() };

        return node;
    }
}

impl GraphNode for SumNode {
    fn id(&self) -> Id {
        self.id
    }

    fn value(&self, graph: &Graph) -> f32 {
        let dependants = graph.get_dependants(self.id);
        let mut acc = 0.;

        dependants
            .iter()
            .for_each(|x| match graph.get_node_value(*x) {
                Some(x) => acc += x,
                None => {}
            });

        return acc;
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<Id, Box<dyn GraphNode>>,
    /// (Source, End)
    pub edges: Vec<(Id, Id)>,
}

impl Graph {
    pub fn new() -> Self {
        return Self {
            nodes: HashMap::new(),
            edges: vec![],
        };
    }

    pub fn add_node(&mut self, node: Box<dyn GraphNode>) -> Id {
        let id = node.id();
        self.nodes.insert(id, node);

        return id;
    }

    pub fn get_dependants(&self, id: Id) -> Vec<Id> {
        self.edges
            .iter()
            .filter(|x| x.1 == id)
            .map(|x| x.0)
            .collect()
    }

    pub fn get_node_value(&self, id: Id) -> Option<f32> {
        match self.nodes.get(&id) {
            Some(node) => Some(node.value(&self)),
            None => None,
        }
    }

    pub fn connect(&mut self, source: Id, end: Id) {
        self.edges.push((source, end));
    }

    pub fn disconnect(&mut self, source: Id, end: Id) {
        self.edges = self
            .edges
            .iter()
            .filter(|x| x.0 != source && x.1 != end)
            .map(|x| *x)
            .collect();
    }

    pub fn remove_node(&mut self, id: Id) {
        self.nodes.remove(&id);
        let dependants = self.get_dependants(id);

        dependants.iter().for_each(|source| self.disconnect(*source, id) );
    }
}
