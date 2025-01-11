use num_bigint::BigUint;

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let r = s.chars().rev().collect::<String>();
    r.parse::<BigUint>().unwrap()
}

fn is_palindrome(n: &BigUint) -> bool {
    let n_r = reverse(n);
    n == &n_r
}

fn is_lychrel(n: &BigUint) -> bool {
    let mut n = n.clone();
    for _ in 0..50 {
        let rev = reverse(&n);
        n = n + rev;
        if is_palindrome(&n) {
            return false;
        }
    }
    true
}

fn main() {
    let lychrel_numbers = (1..10000)
        .map(|x| BigUint::from(x as u32))
        .filter(|n| is_lychrel(n))
        .collect::<Vec<BigUint>>();

    println!("Answer: {}", lychrel_numbers.len());
}
