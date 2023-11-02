use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::data::Point;

/// A complete graph of points
#[derive(Debug, Default, Clone)]
pub struct Map {
    pub nodes: Vec<Point>,
}

#[allow(dead_code, unused_variables)]
impl Map {
    pub fn find_shortest_path_with_intermediate_points_ant_colony(&self, source: &Point, destination: &Point, k: usize) -> Option<(f32, Vec<Point>)> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) || k > self.nodes.len() - 2 {
            return None;
        }

        let nodes: Vec<MapNode> = Self::make_nodes(&self.nodes);

        let source: &MapNode = nodes.iter().find(|node| node.node == source.to_owned()).unwrap();
        let destination: &MapNode = nodes.iter().find(|node| node.node == destination.to_owned()).unwrap();

        let mut distances: HashMap<MapNode, HashMap<MapNode, f32>> = HashMap::new();
        let mut weighted_distances: HashMap<MapNode, Vec<(MapNode, f32)>> = HashMap::new();
        let mut pheromones: HashMap<MapNode, f32> = HashMap::new();

        for node in &nodes {
            distances.insert(node.to_owned(), Self::get_all_distances_for_node(&nodes, node));
            pheromones.insert(node.to_owned(), 1f32);
        }

        for node in &nodes {
            weighted_distances.insert(node.to_owned(), distances[&node].iter().map(|(node, &distance)| (node.to_owned(), (1.0 / (distance + distances[&node][&destination])).powi(19))).collect::<Vec<(MapNode, f32)>>().to_owned());
        }

        let mut rng = thread_rng();

        let mut best_path = (f32::MAX, Vec::<MapNode>::new());

        for iteration_counter in 1..=100 {
            let mut local_best_path = (f32::MAX, Vec::<MapNode>::new());

            for ant_counter in 1..=40 {
                let mut current_path= vec![source.to_owned()];
                let mut current_distance: f32 = 0f32;

                loop {
                    let last = current_path.last().unwrap();

                    if current_path.len() == k - 1 {
                        current_distance += distances[&last][&destination];
                        current_path.push(destination.to_owned());

                        if current_distance < local_best_path.0 {
                            local_best_path = (current_distance, current_path.iter().map(|node| node.to_owned()).collect());
                        }

                        if ant_counter % 1 == 0 {
                            println!("Iteration {} | Ant {} | Distance taken {} | Local best {} | Global best {}", iteration_counter, ant_counter, current_distance, local_best_path.0, best_path.0);
                        }

                        break;
                    }

                    let choice = weighted_distances[&last].iter().map(|(node, weight)| {
                        if current_path.contains(node) || node == destination {
                            (node.to_owned(), 0.0)
                        } else {
                            (node.to_owned(), weight * pheromones[&node].powi(5))
                        }
                    }).collect::<Vec<(MapNode, f32)>>().choose_weighted(&mut rng, |item| item.1).unwrap().to_owned().0;

                    pheromones.insert(choice.to_owned(), pheromones[&choice] * 1.06);

                    current_distance += last.node.distance_to(&choice.node);
                    current_path.push(choice);
                }

                for node in &nodes {
                    let mut current_pheromones = pheromones[&node];
                    if current_pheromones != 1f32 {
                        current_pheromones /= 1.02;

                        if current_pheromones < 1.0 {
                            current_pheromones = 1f32;
                        }
                    }

                    pheromones.insert(node.to_owned(), current_pheromones);
                }

            }

            for node in &nodes {
                pheromones.insert(node.to_owned(), 1f32);
            }

            if local_best_path.0 < best_path.0 {
                best_path = local_best_path;
                // println!("Local best replaced global best");
            }

            for node in &best_path.1 {
                pheromones.insert(node.to_owned(), pheromones[&node] * 1.06f32.powi(5));
            }
        }

        let path = best_path.1.iter().map(|node| node.node.to_owned()).collect();

        Some((best_path.0, path))
    }

    /// Finds approximate shortest path between source and destination
    /// Returns [None] if `k` is too large or if either source or destination [Point] doesn't isn't contained in the map
    pub fn find_shortest_path_with_intermediate_points(&self, source: &Point, destination: &Point, k: usize) -> Option<(f32, Vec<Point>)> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) || k > self.nodes.len() - 2 {
            return None;
        }

        let nodes: Vec<MapNode> = Self::make_nodes(&self.nodes);

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

            let start = &path[best.1 - 1];
            let end = &path[best.1];

            distances.insert(best.0.node.to_owned(), Self::get_all_distances_for_node(&nodes, &best.0.node));

            current_distance -= distances[start][end];
            current_distance += distances[start][&best.0.node];
            current_distance += distances[&best.0.node][end];

            path.insert(best.1, best.0.node.to_owned());

            // println!("Found intermediate point {} - {:?}", path.len() - 2, best.0.node);
        }

        let path: Vec<Point> = path.iter().map(|node| node.node.to_owned()).collect();
        Some((current_distance, path))
    }

    /// Preprocesses nodes to find `k` points closest to a line that directly connects source with destination,
    /// then calls [Self::find_shortest_path_with_intermediate_points] on a new Map with those points
    pub fn find_shortest_path_with_intermediate_points_fast(&self, source: &Point, destination: &Point, k: usize) -> Option<(f32, Vec<Point>)> {
        let mut preprocessed_nodes =  match Self::get_k_closest_to_direct_connection(self, source, destination, k) {
            None => return None,
            Some(nodes) => nodes,
        };

        preprocessed_nodes.push(source.to_owned());
        preprocessed_nodes.push(destination.to_owned());

        Self {
            nodes: preprocessed_nodes
        }.find_shortest_path_with_intermediate_points(&source, &destination, k)
    }

    fn get_points_sorted_by_distance_to_direct_connection(&self, source: &Point, destination: &Point) -> Option<Vec<Point>> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) {
            return None;
        }

        let nodes: Vec<MapNode> = Self::make_nodes(&self.nodes);

        let source: &MapNode = nodes.iter().find(|node| node.node == source.to_owned()).unwrap();
        let destination: &MapNode = nodes.iter().find(|node| node.node == destination.to_owned()).unwrap();

        let mut distances: HashMap<MapNode, HashMap<MapNode, f32>> = HashMap::new();

        distances.insert(source.to_owned(), Self::get_all_distances_for_node(&nodes, source));
        distances.insert(destination.to_owned(), Self::get_all_distances_for_node(&nodes, destination));

        let mut distance_table: BinaryHeap<MapDistanceEntry> = BinaryHeap::new();

        for (node, start_distance) in &distances[source] {
            if let Some(end_distance) = distances[destination].get(&node) {
                if node == source {
                    continue
                }

                distance_table.push(MapDistanceEntry {
                    node: node.to_owned(),
                    distance: start_distance + end_distance,
                })
            }
        }

        Some((0..distance_table.len()).map(|_| distance_table.pop().unwrap().node.node).collect())
    }

    /// Returns `k` points closest to a line that directly connects source with destination
    fn get_k_closest_to_direct_connection(&self, source: &Point, destination: &Point, k: usize) -> Option<Vec<Point>> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) || k > self.nodes.len() - 2 {
            return None;
        }

        let points = match Self::get_points_sorted_by_distance_to_direct_connection(&self, &source, &destination) {
            None => return None,
            Some(points) => points,
        };

        Some(points[..k].to_owned())
    }

    fn make_nodes(points: &Vec<Point>) -> Vec<MapNode> {
        points.iter().enumerate().map(|(iter, point)| {
            MapNode {
                id: iter,
                node: point.to_owned(),
            }
        }).collect()
    }

    fn get_all_distances_for_node(nodes: &Vec<MapNode>, node: &MapNode) -> HashMap<MapNode, f32> {
        let mut hash_map: HashMap<MapNode, f32> = HashMap::new();

        for child in nodes {
            hash_map.insert(
                child.to_owned(),
                node.node.distance_to(&child.node),
            );
        }

        hash_map
    }
}

/// Distinguished and ordered by `distance` reversed for binary heap min-heap use
/// That is to say, an entry of 3.0 distance > an entry of 4.0 distance
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