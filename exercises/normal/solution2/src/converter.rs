pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.split('(').collect();
    if parts.len() != 2 {
        return String::from("Invalid input format");
    }
    let num = parts[0];
    let from_base_str = parts[1].trim_end_matches(')');
    let from_base = from_base_str.parse::<u32>().unwrap_or(10);

    let decimal_value = i32::from_str_radix(num, from_base).unwrap_or(0);

    if to_base < 2 || to_base > 16 {
        return String::from("Invalid target base");
    }

    let mut result = String::new();
    let mut value = decimal_value;

    while value > 0 {
        let remainder = value % to_base as i32;
        value /= to_base as i32;
        result.push(if remainder < 10 {
            (remainder + b'0' as i32) as u8 as char
        } else {
            (remainder - 10 + b'a' as i32) as u8 as char
        });
    }

    result.chars().rev().collect()
}