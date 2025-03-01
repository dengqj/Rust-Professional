pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut sum = 0;
    let mut a = 0;
    let mut b = 1;

    while a < threshold {
        if a % 2 != 0 {
            sum += a;
        }
        let next = a + b;
        a = b;
        b = next;
    }

    sum
}
