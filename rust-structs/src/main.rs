#[derive(Debug)]
struct ExampleStruct {
    name: String,
    age: u32,
    country: String
}

// Define a struct representing a rectangle and then give it a method to calculate the area
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Instantiate an example struct object
    let example = ExampleStruct {
        name: "John Doe".to_string(),
        age: 30,
        country: "USA".to_string()
    };

    // Using the struct update syntax to create a new struct object
    let example2 = ExampleStruct {
        age: 31,
        ..example
    };
    
    // Print the struct objects
    println!("{:?}", example2);

    // Instantiate a Rectangle object
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    // Print the area of the rectangle
    println!("The area of the rectangle is: {}", rect.area());
}
