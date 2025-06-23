use std::env;
use tsp_rust::algs::random_search::random_search;
use tsp_rust::algs::utils::{create_rng_from_seed_string, get_problem_size};
use tsp_rust::cli::utils::{
    read_distance_matrix, validate_cli_arguments,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Random Search", false);

    const NUM_REPETITIONS: usize = 2000;
    let distance_matrix = read_distance_matrix(&args[1]);
    let problem_size = get_problem_size(&distance_matrix);
    let iterations = problem_size * NUM_REPETITIONS;
    let seed = &args[2];
    let mut rng = create_rng_from_seed_string(seed);

    let best_cost = random_search(&mut rng, &distance_matrix, iterations);
    
    println!("Best cost: {}", best_cost);
}
