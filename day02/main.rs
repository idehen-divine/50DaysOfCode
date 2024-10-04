fn main() {
    // Day 2 - Variables and Constants

    // Immutable variable
    let age = 30;
    println!("Age: {}", age);

    // Mutable variable
    let mut height = 180; // in centimeters
    println!("Initial Height: {}", height);
    height = 185; // changing the value
    println!("Updated Height: {}", height);

    // Constant
    const PI: f64 = 3.14159;
    println!("Value of PI: {}", PI);

    // Using a mutable variable in calculations
    let radius = 5;
    let area = PI * (radius * radius) as f64; // Area of a circle
    println!("Area of the circle with radius {}: {}", radius, area);
}
