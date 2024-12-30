fn main() {
    println!("Data types!");

    // data types: integer, float, boolean, character, string

    // variables assign data to temporary memory location, keyword 'let' is used to declare a variable

    // integer
    let a: i32 = 10;
    let b: i64 = 100;

    // sum a and b, nb: a needs to be casted to i64 to be compatible with b
    let sum = a as i64 + b;
    println!("Sum of a and b: {}", sum);

    // float
    let c: f32 = 10.5;
    let d: f64 = 100.5;

    // product c and d, nb: c needs to be casted to f64 to be compatible with d
    let product = c as f64 * d;
    println!("Product of c and d: {}", product);

    // boolean
    let e: bool = true;
    let f: bool = false;

    // logical AND of e and f
    let and = e && f;

    // logical OR of e and f
    let or = e || f;

    println!("Logical AND of e and f: {}", and);
    println!("Logical OR of e and f: {}", or);

    // character
    let g: char = 'A';
    let h: char = 'B';

    // concatenation of g and h
    let concat = format!("{}{}", g, h);

    println!("Concatenation of g and h: {}", concat);

    // string
    let i: String = "Hello".to_string();
}