pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        return 0.0;
    }

    let mut probability = 1.0;
    for i in 1..n {
        probability *= (365 - i as i32) as f64 / 365.0;
    }

    1.0 - probability
}
