use std::fs::File;
use std::io::Write;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Point {
    pub fn save_to_file(&self, file: &mut File) -> std::io::Result<()> {
        let data = format!("Point:{}:{}\n", self.x, self.y);

        file.write_all(data.as_bytes())
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    }
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
#[allow(dead_code)]
pub struct Path {
    pub start: Point,
    pub end: Point,
}

#[allow(dead_code)]
impl Path {
    pub fn save_to_file(&self, file: &mut File) -> std::io::Result<()>{
        let data = format!("Path:{}:{}:{}:{}\n", self.start.x, self.start.y, self.end.y, self.end.y);

        file.write_all(data.as_bytes())
    }
}

#[allow(dead_code)]
pub fn read_from_file(file: &mut File) {
    
}
