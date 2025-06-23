use crate::algs::utils::get_problem_size;
use std::collections::BTreeSet;

pub fn deterministic_greedy(distance_matrix: &Vec<Vec<f64>>) -> f64 {
    let problem_size = get_problem_size(&distance_matrix);

    // Initialize the minimum total distance from city 0 to others
    let mut min_total_distance = (1..problem_size)
        .map(|i| distance_matrix[0][i])
        .sum::<f64>();

    // Find the city with the smallest total distance to all other cities
    let mut most_central_city = 0u32;
    for city in 1..problem_size {
        let total_distance: f64 = distance_matrix[city].iter().sum();
        if total_distance < min_total_distance {
            min_total_distance = total_distance;
            most_central_city = city as u32;
        }
    }

    // Initialize the set of cities to visit and the tour vector
    let mut cities_to_visit: BTreeSet<u32> = (0..problem_size).map(|x| x as u32).collect();
    let mut tour: Vec<u32> = vec![0; problem_size];

    // Start the tour from the most central city
    tour[0] = most_central_city;
    cities_to_visit.remove(&most_central_city);

    // Initialize current city and total distance accumulator
    let mut current_city = most_central_city;
    let mut total_distance = 0.0;

    // Build the tour by always choosing the closest unvisited city
    for i in 1..problem_size {
        let mut iter = cities_to_visit.iter();
        let first_candidate = *iter.next().unwrap();
        let mut closest_city = first_candidate;
        let mut closest_distance = distance_matrix[current_city as usize][closest_city as usize];

        for &candidate in iter {
            let candidate_distance = distance_matrix[current_city as usize][candidate as usize];
            if candidate_distance < closest_distance {
                closest_distance = candidate_distance;
                closest_city = candidate;
            }
        }

        tour[i] = closest_city;
        cities_to_visit.remove(&closest_city);

        total_distance += distance_matrix[current_city as usize][closest_city as usize];
        current_city = closest_city;
    }

    // Complete the tour by returning to the starting city
    total_distance += distance_matrix[tour[problem_size - 1] as usize][most_central_city as usize];

    total_distance
}