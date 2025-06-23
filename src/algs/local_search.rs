use rand_chacha::ChaCha8Rng;
use crate::algs::utils::{compute_cost, generate_random_permutation, get_problem_size};

pub fn local_search(rng: &mut ChaCha8Rng, distance_matrix: &Vec<Vec<f64>>) -> f64 {
    let problem_size = get_problem_size(distance_matrix);
    let mut best_path = generate_random_permutation(rng, problem_size);
    let mut minimum_cost = compute_cost(&best_path, distance_matrix);

    let mut improves = true;

    while improves {
        let mut lowest_local_cost = minimum_cost;
        let mut best_i = 0;
        let mut best_j = 0;

        for i in 0..problem_size {
            for j in i + 1..problem_size {
                best_path.swap(i, j);
                let local_cost = compute_cost(&best_path, distance_matrix);
                best_path.swap(i, j);

                if local_cost < lowest_local_cost {
                    lowest_local_cost = local_cost;
                    best_i = i;
                    best_j = j;
                }
            }
        }

        if lowest_local_cost < minimum_cost {
            minimum_cost = lowest_local_cost;
            best_path.swap(best_i, best_j);
        } else {
            improves = false;
        }
    }

    minimum_cost
}