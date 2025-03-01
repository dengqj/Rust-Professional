use std::collections::{HashMap, HashSet};
use serde_json;
use std::fs;

fn build_adjacency_list(cities_map: &HashMap<String, Vec<String>>) -> HashMap<String, HashSet<String>> {
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();

    for (city, neighbors) in cities_map.iter() {
        let mut unique_neighbors: HashSet<String> = neighbors.iter().cloned().collect();
        if unique_neighbors.len() == 1 && unique_neighbors.contains(city) {
            continue;
        }
        unique_neighbors.insert(city.clone());

        for neighbor in &unique_neighbors {
            let entry = adj_list.entry(neighbor.clone()).or_insert_with(HashSet::new);
            for other in &unique_neighbors {
                if other != neighbor {
                    entry.insert(other.clone());
                }
            }
        }
    }

    adj_list
}

pub fn count_provinces() -> String {
    let json_str = fs::read_to_string("district.json").expect("Failed to read district.json");
    let data: HashMap<String, HashMap<String, Vec<String>>> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    // 收集所有键并排序
    let mut keys: Vec<&String> = data.keys().collect();
    keys.sort(); // 从小到大排序

    let mut results = Vec::new();

    // 按排序后的键顺序处理
    for key in keys {
        let cities_map = data.get(key).unwrap();
        let adj_list = build_adjacency_list(cities_map);

        // DFS 计算连通分量
        let mut visited = HashSet::new();
        let mut province_count = 0;

        fn dfs(city: &str, adj_list: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
            visited.insert(city.to_string());
            if let Some(neighbors) = adj_list.get(city) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        dfs(neighbor, adj_list, visited);
                    }
                }
            }
        }

        for city in adj_list.keys() {
            if !visited.contains(city) {
                dfs(city, &adj_list, &mut visited);
                province_count += 1;
            }
        }

        results.push(province_count.to_string());
    }

    results.join(",")
}