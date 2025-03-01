pub fn goldbach_conjecture() -> String {
    let mut count = 0;
    let mut num = 9; // Start with smallest odd composite
    let mut result = String::new();

    while count < 2 {
        if is_prime(num) {
            num += 2; // Skip primes
            continue;
        }
        let mut can_express = false;
        let mut p = 2;
        while p < num {
            if is_prime(p) {
                let diff = num - p;
                if diff % 2 == 0 && is_square(diff / 2) {
                    can_express = true;
                    break;
                }
            }
            p = next_prime(p);
        }
        if !can_express {
            result.push_str(&format!("{}", num));
            count += 1;
            if count < 2 {
                result.push_str(","); 
            }
        }
        num += 2;
    }

    result
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn next_prime(n: u32) -> u32 {
    let mut next = n + 1;
    while !is_prime(next) {
        next += 1;
    }
    next
}

fn is_square(n: u32) -> bool {
    let sqrt = (n as f64).sqrt() as u32;
    sqrt * sqrt == n
}

