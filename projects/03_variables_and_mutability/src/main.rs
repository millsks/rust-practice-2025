use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // Variables and Mutability Example
    // By default, variables are immutable in Rust
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6; // This line will cause a compile-time error
    // println!("The value of x is: {x}");
    println!("Variables and Mutability Example:");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // This is allowed because x is mutable
    println!("The value of x is: {x}");

    // Constants Example
    // Constants are always immutable and must have a type annotation
    // They can be declared in any scope, including the global scope
    // Constants are conventionally written in uppercase with underscores
    println!("\nConstants Example:");
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing Example
    // Shadowing allows you to declare a new variable with the same name as a previous variable
    // The new variable shadows the previous one
    // This is different from mutability, as the type of the variable can also change
    println!("\nShadowing Example:");
    let x = 5;
    println!("The value of x before shadowing is: {x}");
    let x = x + 1; // x is now 6
    println!("The value of x after first shadowing is: {x}");
    {
        let x = x * 2; // x is now 12 in this inner scope
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}"); // x is still 6

    // Shadowing with Different Types Example
    println!("\nShadowing with Different Types Example:");
    let spaces = "   "; // spaces is a &str
    println!("The value of spaces is: '{spaces}'");
    println!("Type of spaces before shadowing: {}", type_of(&spaces));
    let spaces = spaces.len(); // spaces is now a usize
    println!("The length of spaces is: {spaces}");
    println!("Type of spaces after shadowing: {}", type_of(&spaces));
    // If we tried to do this with mutability instead of shadowing, it would not work:
    // let mut spaces = "   ";
    // spaces = spaces.len(); // This line will cause a compile-time error because the types are different
    // println!("\nThe value of spaces is: '{spaces}'");
}
