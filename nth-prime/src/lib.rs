pub fn nth(n: u32) -> u32 {
    let mut found_primes: Vec<u32> = vec![];
    (2..)
        .filter(|&num| is_prime(&mut found_primes, num))
        .nth(n as usize)
        .unwrap()
}

fn is_prime(found_primes: &mut Vec<u32>, num: u32) -> bool {
    if found_primes.iter().any(|i| num % i == 0) {
        return false;
    }
    found_primes.push(num);
    true
}
