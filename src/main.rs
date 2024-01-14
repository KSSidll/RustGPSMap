use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::Circle;
use plotters::series::LineSeries;
use plotters::style::{BLACK, Color, CYAN, GREEN, RED, ShapeStyle, WHITE};
use plotters::style::full_palette::BLUE;
use data::read_from_file;
use crate::data::Point;

use crate::graph::{Map, Path};

mod data;
mod util;
mod graph;

const DATA_FILENAME: &str = "data.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const K: usize = 70;
    const SOURCE: usize = 2;
    const DESTINATION: usize = 17;

    let (points, _paths) = read_from_file(DATA_FILENAME)?;

    let graph = Map {
        nodes: points.to_owned()
    };

    let source = &graph.nodes[SOURCE];
    let destination = &graph.nodes[DESTINATION];

    println!("source: {:?}", source);
    println!("destination: {:?}", destination);

    const ITERATIONS: i32 = 50;
    const POPULATION: i32 = 100;
    const ALPHA: i32 = 22;
    const BETA: i32 = 5;

    println!("Trying to find a path via ant colony");
    let path = match graph.find_shortest_path_with_intermediate_points_ant_colony(&source, &destination, K, ITERATIONS, POPULATION, ALPHA, BETA) {
        None => None,
        Some(path) => Some(path),
    };

    if path.is_none() {
        println!("No path found")
    } else {
        let path = path.unwrap();

        println!("path distance: {:?}", &path.length);

        let file_path: String = format!("possible_result_{}_{}_{}_{}.png", ITERATIONS, POPULATION, ALPHA, BETA);
        draw(&file_path, &points, &source, &destination, &path)?;
    }

    Ok(())
}

fn draw(file_path: &str, points: &Vec<Point>, source: &Point, destination: &Point, path: &Path) -> Result<(), Box<dyn std::error::Error>>{
    let root = BitMapBackend::new(file_path, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..50f64, 0f64..50f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        LineSeries::new(path.points.iter().map(|point| (point.x as f64, point.y as f64)).collect::<Vec<(f64, f64)>>(), &BLUE)
    )?;

    chart.draw_series(
        points.iter().map(|point| {
            Circle::new(
                (point.x as f64, point.y as f64), 2,
                if point == source {
                    RED.filled()
                } else if point == destination {
                    BLACK.filled()
                } else {
                    GREEN.filled()
                }
            )
        }),
    )?;

    root.present()?;
    println!("Result has been saved to {}", file_path);

    Ok(())
}