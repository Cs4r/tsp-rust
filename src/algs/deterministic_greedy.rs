use crate::algs::utils::get_problem_size;
use std::collections::BTreeSet;

pub fn deterministic_greedy(distance_matrix: Vec<Vec<f64>>) -> f64 {
    let problem_size = get_problem_size(&distance_matrix);

    let mut central_distance: f64 = 0.0;
    let mut most_central_city: u32 = 0;

    // Set city 0 as the reference city to start comparing with the others
    for i in 1..problem_size {
        central_distance += distance_matrix[0][i];
    }

    // Find the most central city
    // (the one whose sum of distances to all other cities is the smallest)
    for i in 1..problem_size {
        let distance: f64 = distance_matrix[i].iter().sum();

        if distance < central_distance {
            central_distance = distance;
            most_central_city = i as u32;
        }
    }

    let mut cities_to_choose: BTreeSet<u32> = (0..problem_size).map(|x| x as u32).collect();
    let mut vec: Vec<u32> = vec![0u32; problem_size];

    vec[0] = most_central_city;
    // Remove the most central city from the set of nodes
    cities_to_choose.remove(&most_central_city);

    let mut closest_city = most_central_city;
    let mut central_distance = 0.0;

    for i in 1..problem_size {
        // Get the first city in the set to initialize the closest search
        let mut iter = cities_to_choose.iter();
        let first = *iter.next().unwrap();
        let mut distance = distance_matrix[closest_city as usize][first as usize];
        let mut selected = first;

        for &city in iter {
            let candidate_distance = distance_matrix[closest_city as usize][city as usize];
            if candidate_distance < distance {
                distance = candidate_distance;
                selected = city;
            }
        }

        closest_city = selected;
        vec[i] = closest_city;
        cities_to_choose.remove(&closest_city);

        // Accumulate distance from previous city
        central_distance += distance_matrix[vec[i - 1] as usize][closest_city as usize];
    }

    // Complete the cycle by returning to the starting city
    central_distance += distance_matrix[vec[problem_size - 1] as usize][most_central_city as usize];

    central_distance
}
