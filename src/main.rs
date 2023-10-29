use std::fs::File;
use data::{Path, Point, read_from_file};

mod data;

const DATA_FILENAME: &str = "data.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(DATA_FILENAME)?;

    let point1 = Point {
        x: 5.1f32,
        y: 3f32,
    };

    let point2 = Point {
        x: 1.1f32,
        y: 7.2f32,
    };

    let path = Path {
        start: point1.clone(),
        end: point2.clone(),
    };

    point1.save_to_file(&mut file)?;
    point2.save_to_file(&mut file)?;
    path.save_to_file(&mut file)?;

    let (points, paths) = read_from_file(DATA_FILENAME)?;

    println!("{:?}", points);
    println!("{:?}", paths);

    Ok(())
}
