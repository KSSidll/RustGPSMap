use std::fs::File;

use data::{Path, Point, read_from_file};

use crate::graph::Map;

mod data;
mod util;
mod graph;

const DATA_FILENAME: &str = "data.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::create(DATA_FILENAME)?;

    const N: usize = 5000;
    const K: usize = 973;
    const SOURCE: usize = 2;
    const DESTINATION: usize = 2;
    const FROM: f32 = 0.0;
    const UNTIL: f32 = 50.0;
    const ACCURACY: f32 = 0.01;

    let points = [(); N].map(|_| {
        Point::generate_random_proportional(FROM, UNTIL, ACCURACY)
    });

    for point in &points {
        point.save_to_file(&mut file)?;
    }

    let path = Path {
        start: points[0].clone(),
        end: points[1].clone(),
    };

    path.save_to_file(&mut file)?;

    let (points, _paths) = read_from_file(DATA_FILENAME)?;

    let graph = Map {
        nodes: points
    };

    let source = &graph.nodes[SOURCE];
    let destination = &graph.nodes[DESTINATION];

    println!("source: {:?}", source);
    println!("destination: {:?}", destination);

    // Good for small sets
    // println!("Trying to find a path via ant colony");
    // let distance = match graph.find_shortest_path_with_intermediate_points_ant_colony(source, destination, K) {
    //     None => None,
    //     Some((distance, _)) => Some(distance),
    // };
    //
    // println!("path distance: {:?}", distance);

    // Ok for small sets ond mid sized sets, low consistency high speed
    println!("Trying to find a path via greedy fast");
    let distance = match graph.find_shortest_path_with_intermediate_points_fast(source, destination, K) {
        None => None,
        Some((distance, _)) => Some(distance),
    };

    println!("path distance: {:?}", distance);

    // Good for small sets, high consistency low speed
    println!("Trying to find a path via greedy normal");
    let distance = match graph.find_shortest_path_with_intermediate_points(source, destination, K) {
        None => None,
        Some((distance, _)) => Some(distance),
    };

    println!("path distance: {:?}", distance);

    Ok(())
}
