use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

struct Coordinate {
    x: f64,
    y: f64,
}

pub fn read_distance_matrix(path: &str) -> Vec<Vec<f64>> {
    let coordinates = read_coordinates(path);
    let problem_size = coordinates.len();

    let mut distance_matrix = vec![vec![0.0; problem_size]; problem_size];

    for i in 0..problem_size {
        for j in i + 1..problem_size {
            let euclidean_distance = euclidean_distance(&coordinates[i], &coordinates[j]);

            distance_matrix[i][j] = euclidean_distance;
            distance_matrix[j][i] = euclidean_distance;
        }
    }

    distance_matrix
}

fn read_coordinates(path: &str) -> Vec<Coordinate> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut problem_size = 0;
    let mut coordinates: Vec<Coordinate> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line_number == 3 {
            let parts: Vec<&str> = to_parts(&line, &":");
            problem_size = str_to_u32(parts[1]);
        } else if line_number >= 6 && (line_number as u32) < problem_size + 6 {
            let parts = to_parts(&line, &" ");

            let x = str_to_f64(parts[1]);
            let y = str_to_f64(parts[2]);

            coordinates.push(Coordinate { x, y });
        }
    }
    coordinates
}

fn euclidean_distance(a: &Coordinate, b: &Coordinate) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn str_to_u32(part: &str) -> u32 {
    str::parse::<u32>(part).unwrap()
}

fn str_to_f64(part: &str) -> f64 {
    str::parse::<f64>(part).unwrap()
}

fn to_parts<'a>(line: &'a str, separator: &'a str) -> Vec<&'a str> {
    line.split(separator).map(|s| s.trim()).collect()
}

pub fn validate_cli_arguments(args: &[String], algorithm: &str, seed_required: bool) {
    let program_name = args.get(0).map_or("program", String::as_str);
    let (header, usage) = generate_cli_help_text(program_name, algorithm, seed_required);

    // Check correct number of arguments based on whether seed is optional
    if (seed_required && args.len() != 3)
        || (!seed_required && !(args.len() == 2 || args.len() == 3))
    {
        eprintln!(
            "{}ERROR: Incorrect number of arguments.\n\n{}",
            header, usage
        );
        process::exit(1);
    }

    let data_file_path = &args[1];
    if !Path::new(data_file_path).exists() {
        eprintln!(
            "{}ERROR: Failed to open data file '{}': File does not exist\nCheck if the file exists and is readable.",
            header, data_file_path
        );
        process::exit(1);
    }

    if seed_required || args.len() == 3 {
        if args.len() < 3 {
            eprintln!("{}ERROR: Seed argument missing.\n\n{}", header, usage);
            process::exit(1);
        }

        let seed_index: usize = args[2].parse().unwrap_or_else(|_| {
            eprintln!(
                "{}ERROR: The seed argument must be a number between 1 and 10.\n\n{}",
                header, usage
            );
            process::exit(1);
        });

        if seed_index < 1 || seed_index > crate::algs::utils::SEEDS.len() {
            eprintln!(
                "{}ERROR: Seed number must be between 1 and 10 (inclusive).\n\n{}",
                header, usage
            );
            process::exit(1);
        }
    }
}

fn generate_cli_help_text(
    program_name: &str,
    algorithm: &str,
    seed_required: bool,
) -> (String, String) {
    let header = format!(
        "
#######################################
#                                     #
#           o-O-o  o-o  o--o          #
#            |   |     |   |          #
#            |    o-o  O--o           #
#            |       | |              #
#            o   o--o  o              #
#                                     #
#######################################
# Travelling Salesman Problem
# Algorithm used: {}\n",
        algorithm
    );

    let seed_arg = if seed_required {
        "<seed_number>"
    } else {
        "[seed_number]"
    };

    let mut usage = format!(
        "Execution: {} <data_file> {}\n\
         <data_file>: .tsp file from TSPLIB95\n\
         {}:  Index   Seed\n",
        program_name, seed_arg, seed_arg
    );

    for (i, seed) in crate::algs::utils::SEEDS.iter().enumerate() {
        usage.push_str(&format!("                 {:>2}     {}\n", i + 1, seed));
    }

    (header, usage)
}
