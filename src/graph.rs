use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use crate::data::{Path, Point};

#[derive(Debug, Default, Clone)]
pub struct PointGraphNode {
    pub node: Point,
    pub connections: Vec<Point>,
}

impl Hash for PointGraphNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state)
    }
}

impl PartialEq for PointGraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for PointGraphNode {}

/*
* Compared by length, lower first
*/
#[derive(Debug, Default, Clone)]
pub struct PointGraphNodePath {
    pub path: Path,
}

impl PartialOrd for PointGraphNodePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.path.length().partial_cmp(&self.path.length())
    }
}

impl Ord for PointGraphNodePath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.path.length().total_cmp(&self.path.length())
    }
}

impl PartialEq for PointGraphNodePath {
    fn eq(&self, other: &Self) -> bool {
        self.path.length() == other.path.length()
    }
}

impl Eq for PointGraphNodePath {}

#[derive(Debug, Default, Clone)]
pub struct PointGraph {
    pub nodes: Vec<PointGraphNode>,
}

#[allow(dead_code)]
impl PointGraph {
    pub fn find_shortest_path_with_intermediate_points(&self, source: PointGraphNode, destination: PointGraphNode, k: i32) -> Option<Vec<PointGraphNode>> {
        if !self.nodes.contains(&source) || !self.nodes.contains(&destination) {
            return None
        }

        let mut distance_table: HashMap<PointGraphNode, f32> = HashMap::new();
        for node in &self.nodes {
            distance_table.insert(node.to_owned(), 0.0);
        }

        let mut heap: BinaryHeap<PointGraphNodePath> = BinaryHeap::new();

        None
    }
}

