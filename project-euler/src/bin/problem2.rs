fn main() {
    // Initialize the first two Fibonacci numbers
    let mut fibonacci = vec![1, 2];

    loop {
        let latest = fibonacci[fibonacci.len() - 1] + fibonacci[fibonacci.len() - 2];

        if latest > 4000000 {
            break;
        }

        fibonacci.push(latest);
    }

    let sum: usize = fibonacci.iter().filter(|&x| x % 2 == 0).sum();

    println!("Answer: {}", sum);
}
