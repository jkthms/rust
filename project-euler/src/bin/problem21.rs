use std::collections::{HashMap, HashSet};

// Return the divisors of a number
fn divisors(n: u32) -> Vec<u32> {
    let mut divisors = vec![];

    for i in 1..n {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    return divisors;
}

fn divisor_sum(divisors: Vec<u32>) -> u32 {
    return divisors.iter().sum::<u32>();
}

fn main() {
    let mut ds: HashMap<u32, u32> = HashMap::new();

    for i in 1..10000 {
        let divisors = divisors(i);
        let sum = divisor_sum(divisors);
        ds.insert(i, sum);
    }

    let mut amicable_numbers = vec![];

    for (key, value) in &ds {
        if ds.contains_key(&value) {
            let sum_other = ds.get(&value).unwrap();
            if key == sum_other {
                if key != value {
                    amicable_numbers.push(key);
                    amicable_numbers.push(value);
                }
            }
        }
    }

    // Filter out duplicates
    let unique_amicable_numbers: HashSet<_> = amicable_numbers.into_iter().collect();

    println!(
        "Answer: {:?}",
        unique_amicable_numbers.iter().copied().sum::<u32>()
    );
}
