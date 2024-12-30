use serde::Deserialize;

// Specify the input for the Fibonacci GET endpoint
#[derive(Deserialize, Debug, Default)]
pub struct FibonacciOptions {
    pub n: i32,
}
