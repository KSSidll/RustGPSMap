use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use crate::data::Point;

#[derive(Debug, Default, Clone)]
pub struct Map {
    pub nodes: Vec<Point>,
}

#[allow(dead_code)]
impl Map {
    /// Finds approximate shortest path between source and destination
    /// Returns [None] if `k` is too large or if either source or destination [Point] doesn't isn't contained in the map
    pub fn find_shortest_path_with_intermediate_points(&self, source: &Point, destination: &Point, k: usize) -> Option<(f32, Vec<Point>)> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) || k > self.nodes.len() - 2 {
            return None;
        }

        let nodes: Vec<MapNode> = self.to_owned().nodes.iter().enumerate().map(|(iter, point)| {
            MapNode {
                id: iter,
                node: point.to_owned(),
            }
        }).collect();

        let source: &MapNode = nodes.iter().find(|node| node.node == source.to_owned()).unwrap();
        let destination: &MapNode = nodes.iter().find(|node| node.node == destination.to_owned()).unwrap();

        let mut distances: HashMap<MapNode, HashMap<MapNode, f32>> = HashMap::new();

        distances.insert(source.to_owned(), Self::get_all_distances_for_node(&nodes, source));
        distances.insert(destination.to_owned(), Self::get_all_distances_for_node(&nodes, destination));

        let mut path: Vec<MapNode> = vec![source.to_owned(), destination.to_owned()];
        let mut current_distance: f32 = distances[source][destination];

        while path.len() - 2 < k {
            let mut best: Option<(MapDistanceEntry, usize)> = None;

            for itr in 1..path.len() {
                let start = &path[itr - 1];
                let end = &path[itr];

                let mut distance_table: BinaryHeap<MapDistanceEntry> = BinaryHeap::new();

                for (node, start_distance) in &distances[start] {
                    if !path.contains(&node) {
                        if let Some(end_distance) = distances[end].get(&node) {
                            distance_table.push(MapDistanceEntry {
                                node: node.to_owned(),
                                distance: start_distance + end_distance,
                            })
                        }
                    }
                }

                let closest = distance_table.peek().unwrap();

                if best == None || best.to_owned().unwrap().0.distance > closest.distance {
                    best = Some((closest.to_owned(), itr))
                }
            }

            let best = best.unwrap();
            path.insert(best.1, best.0.node.to_owned());

            distances.insert(best.0.node.to_owned(), Self::get_all_distances_for_node(&nodes, &best.0.node));

            current_distance = 0f32;
            for itr in 1..path.len() {
                let start = &path[itr - 1].node;
                let end = &path[itr].node;

                current_distance += start.distance_to(end);
            }

            println!("Found intermediate point {} - {:?}", path.len() - 2, best.0.node);
        }

        let path: Vec<Point> = path.iter().map(|node| node.node.to_owned()).collect();
        Some((current_distance, path))
    }

    fn get_all_distances_for_node(nodes: &Vec<MapNode>, node: &MapNode) -> HashMap<MapNode, f32> {
        let mut hash_map: HashMap<MapNode, f32> = HashMap::new();

        for child in nodes {
            if child == node {
                continue;
            }

            hash_map.insert(
                child.to_owned(),
                node.node.distance_to(&child.node),
            );
        }

        hash_map
    }
}

/// Distinguished and ordered by `distance` in reverse
#[derive(Debug, Clone)]
struct MapDistanceEntry {
    pub node: MapNode,
    pub distance: f32,
}

impl PartialEq for MapDistanceEntry {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for MapDistanceEntry {}

impl PartialOrd for MapDistanceEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for MapDistanceEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.total_cmp(&self.distance)
    }
}

/// Distinguished and ordered by `id`
#[derive(Debug, Clone)]
struct MapNode {
    pub id: usize,
    pub node: Point,
}

impl Into<Point> for MapNode {
    fn into(self) -> Point {
        self.node
    }
}

impl Hash for MapNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MapNode {}

impl PartialOrd for MapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for MapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}