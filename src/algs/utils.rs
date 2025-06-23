use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub const SEEDS: [u64; 10] = [
    12345678, 23456781, 34567812, 45678123, 56781234, 67812345, 78123456, 81234567, 87654321,
    18765432,
];

pub fn create_rng_from_seed_string(seed_index_str: &str) -> ChaCha8Rng {
    let seed_index: usize = seed_index_str.parse().unwrap_or(1);
    let seed_index = seed_index.saturating_sub(1);

    let seed_num = SEEDS.get(seed_index).unwrap_or(&SEEDS[0]);

    let mut seed_bytes = [0u8; 32];

    seed_bytes[..8].copy_from_slice(&seed_num.to_le_bytes());

    ChaCha8Rng::from_seed(seed_bytes)
}

pub fn get_problem_size(matrix: &Vec<Vec<f64>>) -> usize {
    matrix.len()
}

pub fn generate_random_permutation(rng: &mut ChaCha8Rng, problem_size: usize) -> Vec<u32> {
    let mut v: Vec<u32> = (0..problem_size).map(|i| i as u32).collect();

    v.shuffle(rng);

    v
}

pub fn compute_cost(path: &Vec<u32>, distance_matrix: &Vec<Vec<f64>>) -> f64 {
    let mut cost: f64 = 0.0;
    let problem_size = path.len();

    for _i in 0..problem_size - 1 {
        cost += distance_matrix[path[_i] as usize][path[_i + 1] as usize];
    }

    cost += distance_matrix[path[problem_size - 1] as usize][path[0] as usize];

    cost
}