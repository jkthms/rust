use std::fs;

fn main() {
    // Read in the file and convert the number to a string
    let file_path = "src/bin/problem8.txt";
    let number_string = fs::read_to_string(file_path).unwrap();

    // Remove all newlines
    let number_string: String = number_string
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    println!("Parsed Input: {}", number_string);

    // Given this input, we need to loop through and find the largest product of 4 consecutive digits
    let mut largest_product = 0;
    let length = 13;

    for i in 0..number_string.len() - length {
        let product = number_string[i..i + length]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .product::<u64>();

        if product > largest_product {
            largest_product = product;
        }
    }

    println!("Answer: {}", largest_product);
}
