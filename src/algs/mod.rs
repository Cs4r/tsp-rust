use rand_chacha::ChaChaRng;
use crate::utils::{generate_random_permutation, recompute_cost};
use crate::utils::compute_cost;


pub fn local_search(rng: &mut ChaChaRng, distance_matrix: &Vec<Vec<f64>>) -> Vec<u32> {
    let problem_size = distance_matrix[0].len();
    let mut best_path = generate_random_permutation(rng, problem_size);
    let mut minimum_cost = compute_cost(&best_path, distance_matrix);
    let mut best_i = 0;
    let mut best_j = 0;

    let mut improves = true;

    while improves {
        let mut lowest_local_cost = minimum_cost;

        for i in 0..problem_size {
            for j in i + 1..problem_size {
                best_path.swap(i, j);
                let local_cost = recompute_cost(&best_path, minimum_cost, distance_matrix, i, j);

                if local_cost < lowest_local_cost {
                    lowest_local_cost = local_cost;
                    best_i = i;
                    best_j = j;
                }

                best_path.swap(i, j);
            }
        }

        if lowest_local_cost < minimum_cost {
            minimum_cost = lowest_local_cost;
            best_path.swap(best_i, best_j);
        } else {
            improves = false;
        }
    }

    best_path
}

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
