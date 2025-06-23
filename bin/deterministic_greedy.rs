use std::env;
use tsp_rust::algs::deterministic_greedy::deterministic_greedy;
use tsp_rust::cli::utils::{read_distance_matrix, validate_cli_arguments};

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Deterministic Greedy");

    let distance_matrix = read_distance_matrix(&args[1]);

    let best_cost = deterministic_greedy(&distance_matrix);

    println!("Best cost: {}", best_cost);
}