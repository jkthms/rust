use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    let mut answer = 0;
    let mut fibonacci: Vec<BigUint> = vec![One::one(), One::one()];

    loop {
        let next = &fibonacci[fibonacci.len() - 1] + &fibonacci[fibonacci.len() - 2];

        if next.to_string().len() >= 1000 {
            answer = fibonacci.len() + 1;
            break;
        }

        fibonacci.push(next);
    }

    println!("Answer: {}", answer);
}
