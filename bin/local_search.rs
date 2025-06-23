use std::env;
use tsp_rust::algs::local_search::local_search;
use tsp_rust::algs::utils::{create_rng_from_seed_string, get_problem_size};
use tsp_rust::cli::utils::{read_distance_matrix, validate_cli_arguments};

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Local Search", true);

    let distance_matrix = read_distance_matrix(&args[1]);
    let seed = &args[2];
    let mut rng = create_rng_from_seed_string(seed);

    let best_cost = local_search(&mut rng, &distance_matrix);

    println!("Best cost: {}", best_cost);
}