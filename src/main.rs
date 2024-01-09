use data::read_from_file;

use crate::graph::Map;

mod data;
mod util;
mod graph;

const DATA_FILENAME: &str = "data.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const K: usize = 31;
    const SOURCE: usize = 2;
    const DESTINATION: usize = 17;

    let (points, _paths) = read_from_file(DATA_FILENAME)?;

    let graph = Map {
        nodes: points
    };

    let source = &graph.nodes[SOURCE];
    let destination = &graph.nodes[DESTINATION];

    println!("source: {:?}", source);
    println!("destination: {:?}", destination);

    println!("Trying to find a path via ant colony");
    let distance = match graph.find_shortest_path_with_intermediate_points_ant_colony(source, destination, K) {
        None => None,
        Some((distance, _)) => Some(distance),
    };

    println!("path distance: {:?}", distance);

    Ok(())
}
