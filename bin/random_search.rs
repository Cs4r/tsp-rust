use std::env;
use tsp_rust::algs::utils::{compute_cost, create_rng_from_seed_string, generate_random_permutation, get_problem_size};
use tsp_rust::cli::utils::{
    read_distance_matrix, validate_cli_arguments,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Random Search");

    const NUM_REPETITIONS: usize = 2000;
    let distance_matrix = read_distance_matrix(&args[1]);
    let problem_size = get_problem_size(&distance_matrix);
    let iterations = problem_size * NUM_REPETITIONS;
    let seed = &args[2];
    let mut rng = create_rng_from_seed_string(seed);

    let mut best_path = generate_random_permutation(&mut rng, problem_size);
    let mut best_cost = compute_cost(&best_path, &distance_matrix);

    for _ in 0..iterations {
        let current_path = generate_random_permutation(&mut rng, problem_size);
        let current_cost = compute_cost(&current_path, &distance_matrix);

        if current_cost < best_cost {
            best_cost = current_cost;
            best_path = current_path;
        }
    }

    println!("Best cost: {}", best_cost);
}
