pub fn new_count_distinct(input_str: &str) -> usize {

    let mut unique_parts = Vec::new();

    for part in input_str.split(',') {
        let trimmed_part = part.trim(); 
        if !trimmed_part.is_empty() { 
            if !unique_parts.iter().any(|existing| existing == trimmed_part) {
                unique_parts.push(trimmed_part.to_string()); 
            }
        }
    }
    unique_parts.len()
}