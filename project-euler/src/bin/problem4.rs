fn is_palindrome(n: usize) -> bool {
    let string_n = n.to_string();
    let reversed_n = string_n.chars().rev().collect::<String>();

    if (string_n == reversed_n) {
        return true;
    }

    return false;
}

fn main() {
    let mut largest_palindrome = 0;

    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;

            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    println!("Answer: {}", largest_palindrome);
}
