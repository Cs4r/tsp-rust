use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coordinate {
    x: f64,
    y: f64,
}

pub fn read_distance_matrix(path: &str) -> Vec<Vec<f64>> {
    let coordinates = read_coordinates_vector(path);
    let problem_size = coordinates.len();

    let mut distance_matrix = square_matrix_of_zeros(problem_size);

    for i in 0..problem_size {
        for j in i + 1..problem_size {
            let euclidean_distance = calculate_euclidean_distance(&coordinates[i], &coordinates[j]);

            distance_matrix[i][j] = euclidean_distance;
            distance_matrix[j][i] = euclidean_distance;
        }
    }

    distance_matrix
}

fn read_coordinates_vector(path: &str) -> Vec<Coordinate> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut problem_size = 0;
    let mut coordinates: Vec<Coordinate> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line_number == 3 {
            let parts: Vec<&str> = to_parts(&line, &":");
            problem_size = str_to_u32(parts[1]);
        } else if line_number >= 6 && line_number < problem_size + 6 {
            let parts = to_parts(&line, &" ");

            let x = str_to_f64(parts[1]);
            let y = str_to_f64(parts[2]);

            coordinates.push(Coordinate { x, y });
        }
    }
    coordinates
}

fn square_matrix_of_zeros(problem_size: usize) -> Vec<Vec<f64>> {
    let mut distance_matrix: Vec<Vec<f64>> = Vec::with_capacity(problem_size);

    for _i in 0..problem_size {
        let row = zeros_vector(problem_size);
        distance_matrix.push(row);
    }
    
    distance_matrix
}

fn calculate_euclidean_distance(a: &Coordinate, b: &Coordinate) -> f64 {
    let diff_coord_x = a.x - b.x;
    let diff_coord_y = a.y - b.y;

    f64::sqrt(diff_coord_x.powf(2.0) + diff_coord_y.powf(2.0))
}

fn zeros_vector(problem_size: usize) -> Vec<f64> {
    let mut row = Vec::with_capacity(problem_size);

    for _ in 0..problem_size {
        row.push(0.0);
    }
    row
}

fn str_to_u32(part: &str) -> usize {
    str::parse::<usize>(part).unwrap()
}

fn str_to_f64(part: &str) -> f64 {
    str::parse::<f64>(part).unwrap()
}

fn to_parts<'a>(line: &'a String, separator: &'a str) -> Vec<&'a str> {
    line.split(separator).map(|s| s.trim()).collect()
}