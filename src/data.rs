use core::fmt;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Write, self, BufRead};
use std::num::ParseFloatError;

const POINT_FORMAT_START: &str = "Point";
const PATH_FORMAT_START: &str = "Path";

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Point {
    pub fn save_to_file(&self, file: &mut File) -> std::io::Result<()> {
        let data = format!("{}:{}:{}\n", POINT_FORMAT_START, self.x, self.y);

        file.write_all(data.as_bytes())
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    }
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Path {
    pub start: Point,
    pub end: Point,
}

#[allow(dead_code)]
impl Path {
    pub fn save_to_file(&self, file: &mut File) -> std::io::Result<()>{
        let data = format!("{}:{}:{}:{}:{}\n", PATH_FORMAT_START, self.start.x, self.start.y, self.end.x, self.end.y);

        file.write_all(data.as_bytes())
    }
}

#[allow(dead_code)]
pub fn read_from_file<P>(path: P) -> Result<(VecDeque<Point>, VecDeque<Path>), ReadError>
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
            },
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
            },
            _ => {}
        }
    }

    Ok((points, paths))
}

#[derive(Debug)]
pub enum ReadError {
    Parse(ParseFloatError),
    IO(io::Error),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::Parse(ref e) => write!(f,"{}", e),
            ReadError::IO(ref e) => write!(f,"{}", e),
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ReadError::Parse(ref e) => Some(e),
            ReadError::IO(ref e) => Some(e),
        }
    }
}

impl From<ParseFloatError> for ReadError {
    fn from(err: ParseFloatError) -> ReadError {
        ReadError::Parse(err)
    }
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::IO(err)
    }
}

