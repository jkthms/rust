use num_bigint::BigUint;

fn main() {
    let mut factorial = BigUint::from(1 as u32);

    for i in 1..100 {
        factorial *= BigUint::from(i as u32);
    }

    let answer = factorial
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    println!("Answer: {}", answer);
}
