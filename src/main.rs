use crate::utils::read_distance_matrix;

mod utils;

fn main() {
    let distance_matrix = read_distance_matrix("resources/berlin52.tsp");

    println!("{:?}", distance_matrix);
}
