use rand_chacha::ChaChaRng;
use crate::utils::generate_random_permutation;
use crate::utils::compute_cost;

pub fn random_search(iterations: u32, rng: &mut ChaChaRng, distance_matrix: &Vec<Vec<f64>>) -> Vec<u32> {
    let problem_size = distance_matrix[0].len();
    let mut best_path = generate_random_permutation(rng, problem_size);
    let mut best_cost = compute_cost(&best_path, distance_matrix);

    let mut costs = Vec::new();

    for _i in 0..iterations {
        let current_path = generate_random_permutation(rng, problem_size);
        let current_cost = compute_cost(&current_path, distance_matrix);

        costs.push(current_cost);

        if current_cost < best_cost {
            best_cost = current_cost;
            best_path = current_path;
        }
    }

    return best_path;
}
