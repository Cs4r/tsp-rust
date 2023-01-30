mod utils;
use crate::utils::read_distance_matrix;
use crate::utils::generate_random_permutation;


fn main() {
    let distance_matrix = read_distance_matrix("./resources/berlin52.tsp");

    println!("{:?}", distance_matrix);

    random_search(100, 288, &distance_matrix);

    ()
}

fn random_search(iterations: u32, seed: u64, distance_matrix: &Vec<Vec<f64>>) -> () {

    let mut path = generate_random_permutation(seed, distance_matrix[0].len());

    ()
}

