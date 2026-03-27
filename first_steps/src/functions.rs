// // C# - Methods in classes
// public class Calculator
// {
//     // Instance method
//     public int Add(int a, int b)
//     {
//         return a + b;
//     }
    
//     // Static method
//     public static int Multiply(int a, int b)
//     {
//         return a * b;
//     }
    
//     // Method with ref parameter
//     public void Increment(ref int value)
//     {
//         value++;
//     }
// }


// Rust - Standalone functions
fn add(a: i32, b: i32) -> i32 {
    a + b  // No 'return' needed for final expression
}
#[allow(dead_code)]
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return is also fine
}
// Function with mutable reference
fn increment(value: &mut i32) {
    *value += 1;
}

// // C# - Statements vs expressions
// public int GetValue()
// {
//     if (condition)
//     {
//         return 42;  // Statement
//     }
//     return 0;       // Statement
// }

// Rust - Everything can be an expression
fn get_value(condition: bool) -> i32 {
    if condition {
        42  // Expression (no semicolon)
    } else {
        0   // Expression (no semicolon)
    }
    // The if-else block itself is an expression that returns a value
}

// Or even simpler
fn get_value_ternary(condition: bool) -> i32 {
    if condition { 42 } else { 0 }
}

pub fn run() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    let mut x = 10;
    increment(&mut x);
    println!("After increment: {}", x);
    let mut value = get_value(true);
    println!("Condition true, then value is 42: {value}");
    value = get_value_ternary(false);
    println!("Condition false, then value is not 42: {value}");
    
}