use std::fs::File;

use data::{Path, Point, read_from_file};

use crate::graph::Map;

mod data;
mod util;
mod graph;

const DATA_FILENAME: &str = "data.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::create(DATA_FILENAME)?;

    const N: usize = 10000;
    const K: usize = 971;
    const SOURCE: usize = 2;
    const DESTINATION: usize = 8;
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

    let res = graph.find_shortest_path_with_intermediate_points(source, destination, K);

    println!("path: {:?}", res);

    Ok(())
}
