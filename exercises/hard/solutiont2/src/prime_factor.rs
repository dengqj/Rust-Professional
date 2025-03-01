use std::collections::HashSet;

// 自定义的简单随机数生成器 (线性同余法)
pub struct SimpleRng {
    seed: u128,
}

impl SimpleRng {
    pub fn new(seed: u128) -> Self {
        SimpleRng { seed }
    }

    pub fn next(&mut self) -> u128 {
        // Constants for a simple linear congruential generator (LCG)
        const A: u128 = 1664525;
        const C: u128 = 1013904223;
        const M: u128 = 1u128 << 32; // 2^32

        self.seed = (A * self.seed + C) % M;
        self.seed
    }

    pub fn next_in_range(&mut self, min: u128, max: u128) -> u128 {
        if min >= max {
              panic!("Invalid range: min must be less than max");
          }
          min + (self.next() % (max - min))
      }
  }



pub fn find_max_prime_factor(mut number: u128) -> u128 {
    if number <= 1 {
        return 0;
    }

    // 小素数表，前50个素数
    const SMALL_PRIMES: &[u128] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
        157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
    ];
    let mut max_factor = 1;

    // 快速除去小素数因子
    for &prime in SMALL_PRIMES {
        while number % prime == 0 {
            max_factor = prime;
            number /= prime;
        }
    }

    if number == 1 {
        return max_factor;
    }

    // GCD函数
    #[inline]
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
    
    //新增函数，Miller Rabin素性测试
    fn miller_rabin(n: u128, k: u32,rng:&mut SimpleRng) -> bool {
        if n <= 1 || n % 2 == 0 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        let mut s = 0;
        let mut d = n - 1;
        while d % 2 == 0 {
            s += 1;
            d /= 2;
        }
        for _ in 0..k {
            let a = rng.next_in_range(2, n-2);
            let mut x = mod_pow(a, d, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            let mut flag = false;
            for _ in 0..s - 1 {
                x = mod_mul(x, x, n);
                if x == n - 1 {
                    flag = true;
                    break;
                }
            }
            if !flag {
                return false;
            }
        }
        true
    }
    
    //新增函数，快速幂算法
    #[inline]
    fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
        let mut result = 1;
        base %= modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = mod_mul(result, base, modulus);
            }
            exp >>= 1;
            base = mod_mul(base, base, modulus);
        }
        result
    }
    
    // 新增函数，快速乘法
    #[inline]
    fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
        (a as u128 * b as u128) % m
    }
    
    //改进的pollard_rho
    fn pollard_rho(n: u128, rng:&mut SimpleRng) -> u128 {
        if n % 2 == 0 {
            return 2;
        }
         if miller_rabin(n, 5,rng) {
            return n; // 如果是素数直接返回自身
        }
        let mut x: u128 = rng.next_in_range(2, n-2);
        let mut y = x;
        let c: u128 = rng.next_in_range(1, n-1);
        let mut d: u128 = 1;
        while d == 1 {
            x = (mod_mul(x,x,n) + c) % n;
            y = (mod_mul(y,y,n)+c)%n;
            y = (mod_mul(y,y,n) + c) % n;

            d = gcd(if x > y { x - y } else { y - x }, n);
        }
        d
    }
    
    let mut factors = HashSet::new();
    // Use current time as seed.
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut rng = SimpleRng::new(current_time);

    // 主分解逻辑
    while number > 1 {
      if number < 1_000_000 { // 小于10^6，用试除
            let mut factor = 3;
            let sqrt_n = (number as f64).sqrt() as u128 + 1;
            while factor <= sqrt_n {
                while number % factor == 0 {
                    factors.insert(factor);
                    number /= factor;
                }
                factor += 2;
            }
            if number > 1 {
               factors.insert(number);
            }
            break;
        }

        let factor = pollard_rho(number,&mut rng);
        if factor == number {
            factors.insert(number);
            break;
        }
        while number % factor == 0 {
            factors.insert(factor);
            number /= factor;
        }
    }

    factors.into_iter().max().unwrap_or(max_factor)
}
