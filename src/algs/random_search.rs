use crate::algs::utils::{compute_cost, generate_random_permutation, get_problem_size, DistanceMatrix};
use rand_chacha::ChaCha8Rng;

pub fn random_search(rng: &mut ChaCha8Rng, distance_matrix: &DistanceMatrix, iterations: usize) -> f64 {

    let problem_size = get_problem_size(&distance_matrix);
    let mut best_path = generate_random_permutation(rng, problem_size);
    let mut best_cost = compute_cost(&best_path, &distance_matrix);

    for _ in 0..iterations {
        let current_path = generate_random_permutation(rng, problem_size);
        let current_cost = compute_cost(&current_path, &distance_matrix);

        if current_cost < best_cost {
            best_cost = current_cost;
            best_path = current_path;
        }
    }

    best_cost
}