use std::fs::File;
use crate::data::{Path, Point};

mod data;

fn main() {
    let mut file = File::create("this.txt").unwrap();

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

    point1.save_to_file(&mut file).unwrap();
    point2.save_to_file(&mut file).unwrap();
    path.save_to_file(&mut file).unwrap();
}
