use std::{env, fs::read_to_string};

use concurrent_prime_sieve::collection::primes_section_concurrently;

pub fn find_rsa(nums: usize) -> (usize, usize) {
    let primes = primes_section_concurrently(2, nums, 4);
    let mut left = 0;
    let mut right = primes.len() - 1;

    while left <= right {
        match nums.cmp(&(primes[left] * primes[right])) {
            std::cmp::Ordering::Equal => return (primes[right], primes[left]),
            std::cmp::Ordering::Less => right -= 1,
            std::cmp::Ordering::Greater => left += 1,
        }
    }
    (primes[right], primes[left])
}
fn main() {
    let f = env::args().nth(1).unwrap();
    let prime = read_to_string(f)
        .unwrap()
        .trim()
        .to_string()
        .parse::<usize>()
        .unwrap();
    let (p, q) = find_rsa(prime);
    println!("{prime}={p}*{q}");
}

#[cfg(test)]
mod tests {
    use crate::find_rsa;

    #[test]
    fn test_find_rsa() {
        assert_eq!(find_rsa(6), (3, 2));
        assert_eq!(find_rsa(77), (11, 7));
        assert_eq!(find_rsa(239821585064027), (15486481, 15485867));
        assert_eq!(find_rsa(2497885147362973), (49979141, 49979141));
    }
}
