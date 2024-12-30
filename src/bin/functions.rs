fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = add(10, 20);
    println!("Sum of 10 and 20: {}", x);

    let y = add(x, 30);
    println!("Sum of x and 30: {}", y);
}