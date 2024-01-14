use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::data::Point;

/// A complete graph of points
#[derive(Debug, Default, Clone)]
pub struct Map {
    pub nodes: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct MapPath {
    pub nodes: Vec<MapNode>,
    pub length: f64,
}

impl Into<Path> for MapPath {
    fn into(self) -> Path {
        Path {
            points: self.nodes.iter().map(|node| node.node.to_owned()).collect(),
            length: self.length
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
    pub length: f64,
}

impl Map {
    pub fn find_shortest_path_with_intermediate_points_ant_colony(&self, source: &Point, destination: &Point, k: usize, iterations: i32, population: i32, alpha: i32, beta: i32) -> Option<Path> {
        if !self.nodes.contains(source) || !self.nodes.contains(destination) || k > self.nodes.len() - 2 {
            return None;
        }

        let nodes: Vec<MapNode> = Self::make_nodes(&self.nodes);

        let source: &MapNode = nodes.iter().find(|node| node.node == source.to_owned()).unwrap();
        let destination: &MapNode = nodes.iter().find(|node| node.node == destination.to_owned()).unwrap();

        let mut distances: HashMap<MapNode, HashMap<MapNode, f64>> = HashMap::new();
        let mut weighted_distances: HashMap<MapNode, Vec<(MapNode, f64)>> = HashMap::new();
        let mut pheromones: HashMap<MapNode, f64> = HashMap::new();

        for node in &nodes {
            distances.insert(node.to_owned(), Self::get_all_distances_for_node(&nodes, node));
            pheromones.insert(node.to_owned(), 1f64);
        }

        for node in &nodes {
            weighted_distances.insert(node.to_owned(), distances[&node].iter().map(|(node, &distance)| (node.to_owned(), (1.0 / (distance + distances[&node][&destination])).powi(alpha))).collect::<Vec<(MapNode, f64)>>().to_owned());
        }

        let mut rng = thread_rng();

        let mut best_path = MapPath { nodes: Vec::new(), length: f64::MAX };

        for iteration_counter in 1..=iterations {
            let mut local_best_path = MapPath { nodes: Vec::new(), length: f64::MAX };

            for ant_counter in 1..=population {
                let mut current_path = vec![source.to_owned()];
                let mut current_distance: f64 = 0f64;

                loop {
                    let last = current_path.last().unwrap();

                    if current_path.len() == k - 1 {
                        current_distance += distances[&last][&destination];
                        current_path.push(destination.to_owned());

                        if current_distance < local_best_path.length {
                            local_best_path = MapPath { nodes: current_path.iter().map(|node| node.to_owned()).collect(), length: current_distance };
                        }

                        if ant_counter % 1 == 0 {
                            println!("Iteration {} | Ant {} | Distance taken {} | Local best {} | Global best {}", iteration_counter, ant_counter, current_distance, local_best_path.length, best_path.length);
                        }

                        break;
                    }

                    let choice = weighted_distances[&last].iter().map(|(node, weight)| {
                        if current_path.contains(node) || node == destination {
                            (node.to_owned(), 0.0)
                        } else {
                            (node.to_owned(), weight * pheromones[&node].powi(beta))
                        }
                    }).collect::<Vec<(MapNode, f64)>>().choose_weighted(&mut rng, |item| item.1).unwrap().to_owned().0;

                    pheromones.insert(choice.to_owned(), pheromones[&choice] * 1.06);

                    current_distance += last.node.distance_to(&choice.node);
                    current_path.push(choice);
                }

                for node in &nodes {
                    let mut current_pheromones = pheromones[&node];
                    if current_pheromones != 1f64 {
                        current_pheromones /= 1.02;

                        if current_pheromones < 1.0 {
                            current_pheromones = 1f64;
                        }
                    }

                    pheromones.insert(node.to_owned(), current_pheromones);
                }
            }

            for node in &nodes {
                pheromones.insert(node.to_owned(), 1f64);
            }

            if local_best_path.length < best_path.length {
                best_path = local_best_path;
                // println!("Local best replaced global best");
            }

            for node in &best_path.nodes {
                pheromones.insert(node.to_owned(), 1.06f64.powi(5));
            }
        }

        Some(best_path.into())
    }

    fn make_nodes(points: &Vec<Point>) -> Vec<MapNode> {
        points.iter().enumerate().map(|(iter, point)| {
            MapNode {
                id: iter,
                node: point.to_owned(),
            }
        }).collect()
    }

    fn get_all_distances_for_node(nodes: &Vec<MapNode>, node: &MapNode) -> HashMap<MapNode, f64> {
        let mut hash_map: HashMap<MapNode, f64> = HashMap::new();

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