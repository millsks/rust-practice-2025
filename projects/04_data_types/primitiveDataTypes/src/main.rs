fn main () {
    // Comprehensive example of primitive data types in Rust
    println!("=== Primitive Data Types in Rust ===\n");
    
    // ===== INTEGER TYPES =====
    println!("--- INTEGER TYPES ---");
    
    // Signed integers (can be negative or positive)
    let i8_val: i8 = -128;          // 8-bit: -128 to 127
    let i16_val: i16 = -32_768;     // 16-bit: -32,768 to 32,767
    let i32_val: i32 = -5;          // 32-bit: -2,147,483,648 to 2,147,483,647 (default)
    let i64_val: i64 = -9_223_372_036_854_775_808; // 64-bit
    let i128_val: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // 128-bit
    
    // Unsigned integers (only positive values and zero)
    let u8_val: u8 = 255;           // 8-bit: 0 to 255
    let u16_val: u16 = 65_535;      // 16-bit: 0 to 65,535
    let u32_val: u32 = 100;         // 32-bit: 0 to 4,294,967,295
    let u64_val: u64 = 18_446_744_073_709_551_615; // 64-bit
    let u128_val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128-bit
    
    // Architecture-dependent integers (size depends on CPU: 32-bit or 64-bit)
    let isize_val: isize = -1000;   // Signed, pointer-sized
    let usize_val: usize = 1000;    // Unsigned, pointer-sized (commonly used for indexing)
    
    println!("Signed integers:");
    println!("  i8:   {}", i8_val);
    println!("  i16:  {}", i16_val);
    println!("  i32:  {}", i32_val);
    println!("  i64:  {}", i64_val);
    println!("  i128: {}", i128_val);
    println!("  isize: {}", isize_val);
    
    println!("\nUnsigned integers:");
    println!("  u8:   {}", u8_val);
    println!("  u16:  {}", u16_val);
    println!("  u32:  {}", u32_val);
    println!("  u64:  {}", u64_val);
    println!("  u128: {}", u128_val);
    println!("  usize: {}", usize_val);
    
    // ===== TYPE SUFFIXES =====
    println!("\n--- TYPE SUFFIXES ---");
    // You can specify the type directly in the literal
    let x = 42u8;       // u8 without annotation
    let y = 100_i32;    // i32 with underscore for readability
    let z = 1_000_000u64; // Large number with underscores and type suffix
    println!("Type suffix examples: {} (u8), {} (i32), {} (u64)", x, y, z);
    
    // ===== NUMBER LITERAL FORMATS =====
    println!("\n--- NUMBER LITERAL FORMATS ---");
    let decimal = 98_222;           // Decimal
    let hex = 0xff;                 // Hexadecimal
    let octal = 0o77;               // Octal
    let binary = 0b1111_0000;       // Binary
    let byte = b'A';                // Byte (u8 only)
    
    println!("Decimal: {}", decimal);
    println!("Hexadecimal (0xff): {}", hex);
    println!("Octal (0o77): {}", octal);
    println!("Binary (0b1111_0000): {}", binary);
    println!("Byte literal (b'A'): {}", byte);
    
    // ===== FLOATING-POINT TYPES =====
    println!("\n--- FLOATING-POINT TYPES ---");
    let f32_val: f32 = 3.14159;     // 32-bit float (single precision)
    let f64_val: f64 = 2.718281828459045; // 64-bit float (double precision, default)
    let f_suffix = 98.6f32;         // f32 with type suffix
    
    println!("f32: {}", f32_val);
    println!("f64: {}", f64_val);
    println!("f32 with suffix: {}", f_suffix);
    
    // ===== BOOLEAN TYPE =====
    println!("\n--- BOOLEAN TYPE ---");
    let t: bool = true;
    let f: bool = false;
    
    // Boolean operations
    let and = t && f;               // AND
    let or = t || f;                // OR
    let not = !t;                   // NOT
    
    println!("true: {}", t);
    println!("false: {}", f);
    println!("true AND false: {}", and);
    println!("true OR false: {}", or);
    println!("NOT true: {}", not);
    
    // ===== CHARACTER TYPE =====
    println!("\n--- CHARACTER TYPE ---");
    let c1: char = 'R';             // ASCII character
    let c2: char = '🦀';            // Emoji (Rust mascot - Ferris the crab!)
    let c3: char = '中';            // Chinese character
    let c4: char = '∞';             // Mathematical symbol
    let c5: char = '\n';            // Escape sequence
    let c6: char = '\u{1F60A}';     // Unicode escape (smiling face)
    
    println!("ASCII: {}", c1);
    println!("Emoji: {}", c2);
    println!("Chinese: {}", c3);
    println!("Math symbol: {}", c4);
    println!("Newline escape: {:?}", c5); // Using {:?} to show escape
    println!("Unicode escape: {}", c6);
    
    // ===== NUMERIC OPERATIONS =====
    println!("\n--- NUMERIC OPERATIONS ---");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;         // Integer division truncates
    let remainder = 43 % 5;
    
    println!("Addition: 5 + 10 = {}", sum);
    println!("Subtraction: 95.5 - 4.3 = {}", difference);
    println!("Multiplication: 4 * 30 = {}", product);
    println!("Division: 56.7 / 32.2 = {}", quotient);
    println!("Truncated division: -5 / 3 = {}", truncated);
    println!("Remainder: 43 % 5 = {}", remainder);
    
    // ===== TYPE BOUNDS =====
    println!("\n--- TYPE BOUNDS (MIN/MAX VALUES) ---");
    println!("i8:  min = {}, max = {}", i8::MIN, i8::MAX);
    println!("u8:  min = {}, max = {}", u8::MIN, u8::MAX);
    println!("i32: min = {}, max = {}", i32::MIN, i32::MAX);
    println!("u32: min = {}, max = {}", u32::MIN, u32::MAX);
    println!("f32: min = {}, max = {}", f32::MIN, f32::MAX);
    println!("f64: min = {}, max = {}", f64::MIN, f64::MAX);
    
    // ===== TYPE CONVERSION (CASTING) =====
    println!("\n--- TYPE CONVERSION (CASTING) ---");
    let a = 3.7_f32;
    let b = a as i32;               // Explicit cast (truncates decimal)
    let c = 100_i32;
    let d = c as f64;               // Cast to float
    
    println!("f32 to i32: {} -> {} (truncated)", a, b);
    println!("i32 to f64: {} -> {}", c, d);
    
    // ===== OVERFLOW BEHAVIOR =====
    println!("\n--- INTEGER OVERFLOW ---");
    println!("In debug mode: overflow causes panic");
    println!("In release mode: overflow wraps around (2's complement)");
    println!("Use wrapping_*, checked_*, saturating_*, or overflowing_* methods for explicit behavior");
    
    let max_u8 = 255u8;
    let wrapped = max_u8.wrapping_add(1);  // Wraps to 0
    let checked = max_u8.checked_add(1);   // Returns None
    let saturating = max_u8.saturating_add(1); // Stays at max
    
    println!("255u8 wrapping_add(1): {}", wrapped);
    println!("255u8 checked_add(1): {:?}", checked);
    println!("255u8 saturating_add(1): {}", saturating);
    
    println!("\n=== End of Primitive Data Types Demo ===");
}