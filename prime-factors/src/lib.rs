pub fn factors(mut n: u64) -> Vec<u64> {
    let mut divisor = 2;
    let mut prime_factors = vec![];
    while n > 1 {
        if n % divisor != 0 {
            divisor += 1;
            continue;
        }
        n /= divisor;
        prime_factors.push(divisor);
    }

    prime_factors
}
