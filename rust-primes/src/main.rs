use std::time::Instant;

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
    is_prime.iter().enumerate().filter(|(_, &p)| p).map(|(i, _)| i).collect()
}

fn is_circular(n: usize, primes: &[usize]) -> bool {
    let digits = n.to_string().chars().collect::<Vec<char>>();
    
    // Check each rotation
    for i in 0..digits.len() {
        let (left, right) = digits.split_at(i);
        let rotation = [right, left].concat();
        let num = rotation.iter().collect::<String>().parse::<usize>().unwrap();
        
        // Return false as soon as we find a non-prime rotation
        if !primes.contains(&num) {
            return false;
        }
    }
    
    // All rotations were prime
    true
}

fn main() {
    // Run a timer
    let start = Instant::now();

    let n = 1000000;

    // Fetch the number of primes below 100,000,00
    let primes = sieve_of_eratosthenes(n);

    // Sum the primes
    let sum: usize = primes.iter().sum();

    println!("Sum of primes below {}: {}", n, sum);

    // Check if a prime is circular (ie. all rotations of the digits are also prime)
    // Do this across multiple threads
    let mut circular_primes = 0;

    for prime in &primes {
        if is_circular(*prime, &primes) {
            circular_primes += 1;
        }
    }

    println!("Number of circular primes below {}: {}", n, circular_primes);

    // Print the time taken
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    // What is the 10,001st prime number?
    println!("The 10,001st prime number is {}", primes[10000]);
}
