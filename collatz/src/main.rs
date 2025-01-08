use std::collections::HashMap;

fn collatz_step(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_length(n: usize, collatz_lengths: &mut HashMap<usize, usize>) -> usize {
    let mut length = 0;
    let mut current = n;
    while current != 1 {
        let next = collatz_step(current);

        if let Some(&cached_length) = collatz_lengths.get(&next) {
            length += cached_length + 1;
            break;
        }

        current = next;
        length += 1;
    }
    collatz_lengths.insert(n, length);
    length
}

fn main() {
    // Initialize the HashMap inside main
    let mut collatz_lengths = HashMap::new();
    collatz_lengths.insert(1, 1);  // Base case

    // Given a number n, find the length of the Collatz sequence for n
    let n = 13;

    println!("The length of the Collatz sequence for {} is {}", n, collatz_length(n, &mut collatz_lengths));

    // Identify the number under 1,000,000 with the longest Collatz sequence
    let mut max_length = 0;
    let mut max_number = 0;
    for i in 1..1000000 {
        let length = collatz_length(i, &mut collatz_lengths);
        if length > max_length {
            max_length = length;
            max_number = i;
        }
    }
    println!("The number under 1,000,000 with the longest Collatz sequence is {} with a length of {}", max_number, max_length);
}
