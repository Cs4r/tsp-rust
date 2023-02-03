mod utils;
mod algs;


use crate::utils::{compute_cost, read_distance_matrix};
use crate::algs::random_search;
use crate::algs::local_search;
use rand::{SeedableRng};
use rand_chacha::ChaChaRng;


fn main() {
    let distance_matrix = read_distance_matrix("./resources/berlin52.tsp");

    let mut rng: ChaChaRng = ChaChaRng::seed_from_u64(288);

    let random_solution = random_search(100, &mut rng, &distance_matrix);

    println!("Random search best cost: {}", compute_cost(&random_solution, &distance_matrix));

    let local_search_solution = local_search(&mut rng, &distance_matrix);

    println!("Local search best cost: {}", compute_cost(&local_search_solution, &distance_matrix));

    ()
}
