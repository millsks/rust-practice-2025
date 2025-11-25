// Compound Data Types in Rust
// This file demonstrates the use of compound data types in Rust, such as tuples, arrays, strings, and slices.
// Compound data types allow you to group multiple values together, which can be of different types.
// Tuples can hold a fixed number of elements, each of potentially different types.
// Arrays hold a fixed number of elements of the same type.
// Slices are dynamically sized views into contiguous sequences of elements.
// Strings are collections of characters, and they can be mutable or immutable.

fn main() {
    // Example of a tuple
    let person: (&str, String, i32, f64, bool) = ("Alice", String::from("Bob"), 30, 5.5, false);
    println!("Tuple: {:?}", person);
    println!("Name1: {}, Name2: {}, Age: {}, Height: {}, Is Student: {}", person.0, person.1, person.2, person.3, person.4);

    // Example of an array
    // {} represents a display format for arrays
    // {?} is used to print the array in a debug format
    // Arrays in Rust are fixed-size and must have a type specified
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);

    // Example of an array of strings
    let words: [&str; 3] = ["Rust", "is", "awesome"];
    println!("Words: {:?}", words);
    // Accessing elements in an array
    println!("First word: {}", words[0]);
    println!("Second word: {}", words[1]);
    println!("Third word: {}", words[2]);


    // Example of a slice
    let slice1: &[i32] = &numbers[1..4];
    let slice2: &[i32] = &numbers[0..2];
    let slice3: &[i32] = &numbers[2..];
    let slice4: &[i32] = &numbers[..]; // Full slice of the array
    let slice5: &[char] = &person.0.chars().collect::<Vec<char>>()[..]; // Slicing the characters of the name in the tuple
    let slice6: &[char] = &person.1.chars().collect::<Vec<char>>()[..]; // Slicing the characters of the name in the tuple
    println!("Slice1 of numbers: {:?}", slice1);
    println!("Slice2 of numbers: {:?}", slice2);
    println!("Slice3 of numbers: {:?}", slice3);
    println!("Slice4 of numbers: {:?}", slice4);
    println!("Slice5 of person name1: {:?}", slice5);
    println!("Slice6 of person name2: {:?}", slice6);

    // Example of a string
    let mut greeting: String = String::from("Hello, ");
    greeting.push_str("World!");
    println!("Greeting: {}", greeting);

    // Strings vs String slices (&str)
    // String is a growable, heap-allocated data structure [growable, mutable, owned]
    // &str is a string slice, which is a view into a string [fixed size, immutable, borrowed]
    let greeting_slice: &str = &greeting[..];
    println!("Greeting slice: {}", greeting_slice);

    // mut is used to make a variable mutable
    let mut mutable_string: String = String::from("Mutable String");
    mutable_string.push_str(" - Now I can change it!");
    println!("Mutable String: {}", mutable_string);
}
