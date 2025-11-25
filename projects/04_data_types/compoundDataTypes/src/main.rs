// Compound Data Types in Rust
// This file demonstrates the use of compound data types in Rust, such as tuples, arrays, strings, and slices.
// Compound data types allow you to group multiple values together, which can be of different types.
// Tuples can hold a fixed number of elements, each of potentially different types.
// Arrays hold a fixed number of elements of the same type.
// Slices are dynamically sized views into contiguous sequences of elements.
// Strings are collections of characters, and they can be mutable or immutable.

fn main() {
    println!("=== Compound Data Types in Rust ===\n");
    
    // ===== TUPLES =====
    println!("--- TUPLES ---");
    
    // Basic tuple
    let person: (&str, String, i32, f64, bool) = ("Alice", String::from("Bob"), 30, 5.5, false);
    println!("Tuple: {:?}", person);
    println!("Name1: {}, Name2: {}, Age: {}, Height: {}, Is Student: {}", person.0, person.1, person.2, person.3, person.4);
    
    // Tuple destructuring - unpacking values into separate variables
    let (name1, name2, age, height, is_student) = person;
    println!("\nDestructured tuple:");
    println!("  name1: {}", name1);
    println!("  name2: {}", name2);
    println!("  age: {}", age);
    println!("  height: {}", height);
    println!("  is_student: {}", is_student);
    
    // Unit type - empty tuple (used when no value is returned)
    let unit: () = ();
    println!("\nUnit type: {:?} (size: {} bytes)", unit, std::mem::size_of_val(&unit));
    
    // Nested tuples
    let nested: ((i32, i32), (i32, i32)) = ((1, 2), (3, 4));
    println!("Nested tuple: {:?}", nested);
    println!("Access nested: ({}, {})", nested.0.0, nested.1.1);
    
    // Tuples as function return values
    let coords = get_coordinates();
    println!("Function returned tuple: {:?}", coords);
    
    // ===== ARRAYS =====
    println!("\n--- ARRAYS ---");
    
    // Basic array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    
    // Array initialization shortcut - [value; count]
    let zeros: [i32; 5] = [0; 5];  // Creates [0, 0, 0, 0, 0]
    let threes: [i32; 10] = [3; 10]; // Creates 10 threes
    println!("Array of zeros: {:?}", zeros);
    println!("Array of threes: {:?}", threes);
    
    // Array length and iteration
    println!("\nArray length: {}", numbers.len());
    println!("Iterating over array:");
    for (index, value) in numbers.iter().enumerate() {
        println!("  Index {}: {}", index, value);
    }
    
    // Mutable arrays
    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("\nOriginal array: {:?}", mutable_array);
    mutable_array[0] = 10;
    mutable_array[4] = 50;
    println!("Modified array: {:?}", mutable_array);
    
    // Multidimensional arrays
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];
    println!("\n2D array (matrix): {:?}", matrix);
    println!("Element at [1][2]: {}", matrix[1][2]);
    
    // Array of strings
    let words: [&str; 3] = ["Rust", "is", "awesome"];
    println!("\nWords: {:?}", words);
    println!("First word: {}", words[0]);
    println!("Second word: {}", words[1]);
    println!("Third word: {}", words[2]);
    
    // Note: Array bounds checking
    // Uncommenting the line below will cause a panic at runtime:
    // println!("Out of bounds: {}", words[10]); // PANIC!
    println!("\nNote: Accessing out-of-bounds index causes panic at runtime");
    println!("Rust checks array bounds to prevent memory safety issues");

    // ===== SLICES =====
    println!("\n--- SLICES ---");
    
    let slice1: &[i32] = &numbers[1..4];  // Elements 1, 2, 3
    let slice2: &[i32] = &numbers[0..2];  // Elements 0, 1
    let slice3: &[i32] = &numbers[2..];   // From element 2 to end
    let slice4: &[i32] = &numbers[..3];   // From start to element 2
    let slice5: &[i32] = &numbers[..];    // Full slice of the array
    
    println!("Original array: {:?}", numbers);
    println!("Slice [1..4]: {:?}", slice1);
    println!("Slice [0..2]: {:?}", slice2);
    println!("Slice [2..]:  {:?}", slice3);
    println!("Slice [..3]:  {:?}", slice4);
    println!("Slice [..]:   {:?}", slice5);
    
    // Slice length
    println!("Slice1 length: {}", slice1.len());
    
    // ===== STRINGS =====
    println!("\n--- STRINGS ---");
    
    // String vs &str
    // String: growable, heap-allocated, mutable, owned
    // &str: string slice, fixed size, immutable, borrowed (view into string)
    
    let mut greeting: String = String::from("Hello, ");
    greeting.push_str("World!");
    println!("String: {}", greeting);
    
    // String methods
    println!("\nString methods:");
    println!("  Length: {}", greeting.len());
    println!("  Is empty: {}", greeting.is_empty());
    println!("  Bytes: {}", greeting.bytes().count());
    println!("  Contains 'World': {}", greeting.contains("World"));
    
    // String indexing - IMPORTANT: Cannot directly index strings!
    // This would NOT work: greeting[0]
    // Strings are UTF-8 encoded, so indexing by byte position isn't safe
    println!("\nString characters (using .chars()):");
    for (i, ch) in greeting.chars().enumerate() {
        println!("  Position {}: {}", i, ch);
    }
    
    // String slicing (careful with UTF-8!)
    let greeting_slice: &str = &greeting[0..5]; // "Hello"
    println!("\nString slice [0..5]: {}", greeting_slice);
    
    // String concatenation methods
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    
    // Method 1: + operator (takes ownership of s1)
    let s3 = s1 + " " + &s2; // s1 is moved here
    println!("\nConcatenation with +: {}", s3);
    
    // Method 2: format! macro (doesn't take ownership)
    let s4 = String::from("Rust");
    let s5 = String::from("Programming");
    let s6 = format!("{} {}", s4, s5);
    println!("Concatenation with format!: {}", s6);
    println!("s4 still valid: {}", s4); // s4 not moved
    
    // Mutable string operations
    let mut mutable_string: String = String::from("Mutable String");
    println!("\nOriginal: {}", mutable_string);
    
    mutable_string.push_str(" - Now I can change it!");
    println!("After push_str: {}", mutable_string);
    
    mutable_string.push('!');
    println!("After push (char): {}", mutable_string);
    
    // String to bytes
    println!("\nString as bytes: {:?}", greeting.as_bytes());
    
    // ===== TUPLE STRUCTS =====
    println!("\n--- TUPLE STRUCTS ---");
    
    // Tuple structs are named tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);
    
    println!("Color RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {})", origin.0, origin.1);
    
    // ===== NESTED STRUCTURES =====
    println!("\n--- NESTED STRUCTURES ---");
    
    // Nested arrays and tuples
    let nested_array: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
    println!("Nested array: {:?}", nested_array);
    
    let complex: (String, [i32; 3], (bool, f64)) = (
        String::from("Complex"),
        [1, 2, 3],
        (true, 3.14)
    );
    println!("Complex tuple: ({}, {:?}, {:?})", complex.0, complex.1, complex.2);
    
    println!("\n=== End of Compound Data Types Demo ===");
}

// Helper function demonstrating tuple as return type
fn get_coordinates() -> (f64, f64) {
    (10.5, 20.3)
}
