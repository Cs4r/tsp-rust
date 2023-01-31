mod utils;
mod algs;


use crate::utils::read_distance_matrix;
use crate::algs::random_search;
use rand::{SeedableRng};
use rand_chacha::ChaChaRng;


fn main() {
    let distance_matrix = read_distance_matrix("./resources/berlin52.tsp");

    let mut rng: ChaChaRng = ChaChaRng::seed_from_u64(288);

    random_search(100, &mut rng, &distance_matrix);

    ()
}
