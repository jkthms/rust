fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let number = 600851475143;
    let upper_limit = (number as f64).sqrt() as usize;

    // Fetch all possible primes
    let primes = sieve_of_eratosthenes(upper_limit);

    // Iterate over the primes in reverse order
    for prime in primes.iter().rev() {
        if number % prime == 0 {
            println!("Answer: {}", prime);
            break;
        }
    }
}
