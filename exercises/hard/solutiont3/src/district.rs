use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn count_provinces() -> String {
    let data = fs::read_to_string("district.json").expect("Failed to read file");
    let json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");

    let mut results = Vec::new();
    for (_, province_data) in json.as_object().unwrap() {
        let province_count = count_provinces_in_batch(province_data.as_object().unwrap());
        results.push(province_count);
    }
    results.join(", ")
}

fn count_provinces_in_batch(province_data: &serde_json::Map<String, Value>) -> usize {
    let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();
    for (city, neighbors) in province_data {
        let neighbors_set: HashSet<String> = neighbors.as_array().unwrap()
            .iter()
            .filter_map(|n| n.as_str())
            .map(|s| s.to_string())
            .collect();
        adjacency_list.insert(city.clone(), neighbors_set);
    }

    // Iteratively build connections until no new connections are found
    loop {
        let mut changed = false;
        let mut new_adjacency_list: HashMap<String, HashSet<String>> = adjacency_list.clone();
        for (city, neighbors) in &adjacency_list {
            let mut new_neighbors = neighbors.clone();
            for neighbor in neighbors {
                if let Some(n_neighbors) = adjacency_list.get(neighbor) {
                    new_neighbors.extend(n_neighbors.clone());
                }
            }
            if new_neighbors.len() > neighbors.len() {
                new_adjacency_list.insert(city.clone(), new_neighbors);
                changed = true;
            }
        }
        adjacency_list = new_adjacency_list;
        if !changed {
            break;
        }
    }

    let mut visited = HashSet::new();
    let mut count = 0;
    for city in adjacency_list.keys() {
        if !visited.contains(city) {
            count += 1;
            depth_first_search(city, &adjacency_list, &mut visited);
        }
    }
    count
}

fn depth_first_search(city: &str, adjacency_list: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
    let mut stack = vec![city.to_string()];
    while let Some(current_city) = stack.pop() {
        if !visited.contains(&current_city) {
            visited.insert(current_city.clone());
            if let Some(neighbors) = adjacency_list.get(&current_city) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
    }
}