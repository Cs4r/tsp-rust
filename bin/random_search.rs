use std::env;
use tsp_rust::utils::validate_cli_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    validate_cli_arguments(&args, "Random Search");

}
