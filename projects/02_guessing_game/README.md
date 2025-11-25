# Guessing Game

A command-line number guessing game that demonstrates fundamental Rust concepts including user input, control flow, error handling, and type conversion.

## Application Summary

The guessing game is an interactive program where:
1. The program generates a random secret number between 1 and 10
2. The player is prompted to guess the number
3. The program provides feedback ("Too low", "Too high", or "Correct")
4. The game continues until the player guesses correctly

## Key Rust Concepts Demonstrated

### 1. External Crate and Standard Library Usage

The program uses:
- `rand` crate for random number generation
- `std::io` module from the standard library for input/output

```rust
use rand::Rng;
use std::io;
```

**Note**: In Rust terminology:
- **Crate**: A compilation unit (like a library or binary)
- **Module**: A namespace within a crate (like `io` within the `std` crate)
- **Package**: A Cargo concept containing one or more crates

The `rand` crate is an external dependency, while `std` is the standard library crate.

### 2. Variable Definitions

```rust
let secret_number = rand::thread_rng().gen_range(1..=10);  // Immutable random number
let mut guess = String::new();                              // Mutable variable (note the 'mut' keyword)
```

- **Immutable by default**: `secret_number` cannot be changed once set
- **Random number generation**: `thread_rng()` creates a random number generator, `gen_range(1..=10)` generates a number from 1 to 10 (inclusive)
- **Mutable with `mut`**: `guess` can be modified to accept user input
- **Shadowing**: The variable `guess` is shadowed when converted from `String` to `u32`

**Understanding Shadowing:**

Shadowing is different from type casting or reassignment in other languages. When you shadow a variable in Rust, you declare a *new* variable with the same name:

```rust
let mut guess = String::new();        // guess is a String
io::stdin().read_line(&mut guess);    

let guess: u32 = guess.trim().parse() // NEW variable named guess (u32)
    .expect("...");                   // shadows the String version
```

The new `guess` (type `u32`) completely shadows the old `guess` (type `String`). The original String variable still exists in memory but is now inaccessible.

**Checking Variable Types:**

You can verify variable types in several ways:

1. **Using `std::any::type_name` (runtime inspection)**:
   ```rust
   fn type_of<T>(_: &T) -> &'static str {
       std::any::type_name::<T>()
   }
   
   let guess = String::new();
   println!("Type: {}", type_of(&guess));  // alloc::string::String
   
   let guess: u32 = 42;
   println!("Type: {}", type_of(&guess));  // u32
   ```

2. **Intentional compiler error (development trick)**:
   ```rust
   let guess = String::new();
   let () = guess;  // Compiler error shows actual type
   ```

3. **Using the `dbg!` macro**:
   ```rust
   dbg!(&guess);  // Shows variable name, location, and value
   ```

4. **IDE hover** - rust-analyzer shows type information on hover

### 3. Control Flow

**Infinite Loop with `loop`**:
```rust
loop {
    // Game logic repeats until 'break' is called
}
```

**Pattern Matching with `match`**:
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a valid number!");
        continue;
    }
};
```
- Handles the `Result` type from `parse()`
- `Ok(num)` - successful parse, extract the number
- `Err(_)` - parse failed, prompt again

**Conditional Logic with `match` and `Ordering`**:
```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too low! Try again."),
    Ordering::Greater => println!("Too high! Try again."),
    Ordering::Equal => {
        println!("Congratulations! You guessed the correct number: {}", secret_number);
        break;  // Exit the loop - without this, the game never ends!
    }
}
```
- Uses `cmp()` method to compare two values, returning an `Ordering` enum
- `break` is **essential** - without it, the infinite loop continues even after winning

**Early Loop Continuation with `continue`**:
- Used for input validation (invalid number or out of bounds)
- Skips the rest of the loop iteration and starts over

### 4. Error Handling

**Method 1: `expect()` for critical errors**:
```rust
.expect("Failed to read line");
```
- Program panics (crashes) if reading from stdin fails

**Method 2: `match` for recoverable errors**:
```rust
match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
}
```
- Gracefully handles invalid input without crashing

### 5. Type Conversion

**String to integer conversion**:
```rust
let guess: u32 = guess.trim().parse()...
```
- `.trim()` removes whitespace (including newline)
- `.parse()` converts string to the specified type (`u32`)
- Type annotation (`: u32`) tells Rust what type to parse into

### 6. User Input

**Reading from standard input**:
```rust
io::stdin()
    .read_line(&mut guess)
```
- `stdin()` returns a handle to standard input
- `read_line()` appends input to the mutable string reference `&mut guess`

## Building and Running

**Run the game**:
```bash
cargo run
```

**Build without running**:
```bash
cargo build
```

**Build optimized release version**:
```bash
cargo build --release
```

## Example Gameplay

```
Welcome to the Guessing Game!
Please enter your guess (a number between 1 and 10):
5
Too high! Try again.
3
Too low! Try again.
4
Congratulations! You guessed the correct number: 4
```

## Future Enhancements

- Add a guess counter to track attempts
- Implement difficulty levels with different number ranges (easy: 1-10, medium: 1-50, hard: 1-100)
- Add option to play multiple rounds
- Display statistics (average guesses, best score, etc.)
