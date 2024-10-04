fn main() {
    // Day 3 - Basic Data Types in Rust

    // Integers
    let age: i32 = 25;
    println!("Age: {}", age);

    // Floats
    let price: f64 = 10.99;
    println!("Price: ${:.2}", price);

    // Booleans
    let is_student: bool = true;
    println!("Is student? {}", is_student);

    // Characters
    let grade: char = 'A';
    println!("Grade: {}", grade);

    // Strings
    let name = String::from("Mart");
    println!("Name: {}", name);

    // Tuples
    let person: (&str, i32) = ("Alice", 30);
    println!("Person: Name = {}, Age = {}", person.0, person.1);

    // Arrays
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Numbers: {:?}", numbers);

    // Vectors
    let mut vector_numbers = vec![1, 2, 3];
    vector_numbers.push(4); // Add an element
    println!("Vector Numbers: {:?}", vector_numbers);
}
