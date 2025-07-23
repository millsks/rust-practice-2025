fn main () {
    // Example of different data types in Rust
    println!("Data Types in Rust:");
    
    // Signed integer, unsigned integer, boolean, float, and character
    // Using explicit type annotations for clarity
    // Note: Rust infers types, but explicit annotations can help readability

    // i32 is a signed 32-bit integer
    // u64 is an unsigned 64-bit integer
    // bool is a boolean type (true or false)
    // f64 is a 64-bit floating point number
    // char is a single Unicode character

    // range of i32 is from -2,147,483,648 to 2,147,483,647
    // range of u64 is from 0 to 18,446,744,073709,551,615
    // f64 can represent a wide range of decimal values
    // char can represent any Unicode character, including emojis
    
    let i1: i32 = -5;
    let u1: u64 = 100;
    let b1: bool = true;
    let f1: f64 = 3.14;
    let c1: char = 'R';

    println!("Signed Integer: {}", i1);
    println!("Unsigned Integer: {}", u1);
    println!("Boolean: {}", b1);
    println!("Float: {}", f1);
    println!("Character: {}", c1);
}