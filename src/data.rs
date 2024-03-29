use core::fmt;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::num::ParseFloatError;
use std::result;

use crate::util::generate_random_f32;

#[derive(Debug)]
pub enum Error {
    Parse(ParseFloatError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parse(ref e) => write!(f, "{}", e),
            Error::IO(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Parse(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::Parse(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

const POINT_FORMAT_START: &str = "Point";
const PATH_FORMAT_START: &str = "Path";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Point {
    pub fn save_to_file(&self, file: &mut File) -> Result<()> {
        let data = format!("{}:{}:{}\n", POINT_FORMAT_START, self.x, self.y);

        Ok(file.write_all(data.as_bytes())?)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt() as f64
    }

    pub fn generate_random(from_x: f32, until_x: f32, accuracy_x: f32, from_y: f32, until_y: f32, accuracy_y: f32) -> Point {
        Point {
            x: generate_random_f32(from_x, until_x, accuracy_x),
            y: generate_random_f32(from_y, until_y, accuracy_y),
        }
    }

    pub fn generate_random_proportional(from: f32, until: f32, accuracy: f32) -> Point {
        Point::generate_random(from, until, accuracy, from, until, accuracy)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Path {
    pub start: Point,
    pub end: Point,
}

#[allow(dead_code)]
impl Path {
    pub fn save_to_file(&self, file: &mut File) -> Result<()> {
        let data = format!("{}:{}:{}:{}:{}\n", PATH_FORMAT_START, self.start.x, self.start.y, self.end.x, self.end.y);

        Ok(file.write_all(data.as_bytes())?)
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(&self.end)
    }
}

pub fn read_from_file<P>(path: P) -> Result<(Vec<Point>, Vec<Path>)>
    where P: AsRef<std::path::Path>, {
    let mut points: VecDeque<Point> = VecDeque::new();
    let mut paths: VecDeque<Path> = VecDeque::new();

    let file = File::open(path)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let values: Vec<&str> = line.split(':').collect();

        match values[0] {
            POINT_FORMAT_START => {
                points.push_back(Point {
                    x: values[1].parse()?,
                    y: values[2].parse()?,
                });
            }
            PATH_FORMAT_START => {
                paths.push_back(Path {
                    start: Point {
                        x: values[1].parse()?,
                        y: values[2].parse()?,
                    },
                    end: Point {
                        x: values[3].parse()?,
                        y: values[4].parse()?,
                    },
                });
            }
            _ => {}
        }
    }

    Ok((Vec::from(points), Vec::from(paths)))
}
