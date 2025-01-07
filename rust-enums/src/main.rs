use rand::{thread_rng, Rng};

// Define an enum with different variants, which can carry additional data
enum Weather {
    Sunny(f32),
    Rainy(i32),
    Cloudy,
    Snowy(f32)
}

// General function to handle the outcome of the weather enum
fn print_weather(weather: &Weather) {
    match weather {
        Weather::Sunny(temp) => println!("Today is sunny with a temperature of {}°C", temp),
        Weather::Rainy(rain) => println!("Today is rainy with {}mm of rain", rain),
        Weather::Cloudy => println!("Today is cloudy"),
        Weather::Snowy(snow) => println!("Today is snowy with {}cm of snow", snow),
    }
}

fn weather() {
    // Create instances of the enum with different variants, such as Sunny with a temperature of 43.2°C
    let today = Weather::Sunny(43.2);
    let tomorrow = Weather::Rainy(10);

    // Note: The match statement must be exhaustive, and cover all possible cases

    // Use match to handle different weather conditions
    print_weather(&today);
    print_weather(&tomorrow);
}

// How to use match to handle exceptions such as a division by zero
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    IncorrectInput,
    UnknownError
}

fn divide(a: f32, b: f32) -> Result<f32, MathError> {
    if b == 0.0 {
        // Return an error if the denominator is zero
        return Err(MathError::DivisionByZero);
    } else if (a < 0.0) || (b < 0.0) {
        // For some reason, do not support negative numbers
        return Err(MathError::IncorrectInput);
    }
    
    // Randomly throw an error also
    if thread_rng().gen_range(0..=1) == 0 {
        // Return an error with no type, which is a catch-all case
        return Err(MathError::UnknownError);
    } else {
        return Ok(a / b);
    }
}

fn exception_handling() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("{}", result),
        // Handle any runtime errors which may have occurred
        Err(MathError::DivisionByZero) => println!("DivisionByZero Detected"),
        Err(MathError::IncorrectInput) => println!("IncorrectInput Detected"),
        // Catch-all case for any other error
        Err(_) => println!("Unknown Error Detected"),
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("{}", result),
        Err(MathError::DivisionByZero) => println!("DivisionByZero Detected"),
        Err(MathError::IncorrectInput) => println!("IncorrectInput Detected"),
        // Catch-all case for any other error
        Err(_) => println!("Unknown Error Detected"),
    }

    match divide(-10.0, 2.0) {
        Ok(result) => println!("{}", result),
        Err(MathError::DivisionByZero) => println!("DivisionByZero Detected"),
        Err(MathError::IncorrectInput) => println!("IncorrectInput Detected"),
        // Catch-all case for any other error
        Err(_) => println!("Unknown Error Detected"),
    }
}

fn main() {
    // Run the weather function
    weather();

    // Run the exception handling function
    exception_handling();
}
