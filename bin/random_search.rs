use std::env;
use tsp_rust::utils::{
    get_number_of_iterations, get_problem_size, read_distance_matrix, validate_cli_arguments,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Random Search");

    let distance_matrix = read_distance_matrix(&args[1]);
    let problem_size = get_problem_size(&distance_matrix);
    let iterations = get_number_of_iterations(&distance_matrix);
}
