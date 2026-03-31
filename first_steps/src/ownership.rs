// Rust - Explicit ownership management
#![allow(dead_code, unused)]

fn process_data() {
    let data = vec![1, 2, 3, 4, 5]; // data owns the vector
    process_list(data); // Ownership moved to function
    // println!("{:?}", data);       // ❌ Error: data no longer owned here
}

fn process_list(mut list: Vec<i32>) {
    // list now owns the vector
    list.push(6);
    // list is dropped here when function ends
}

// Rust - Ownership-aware swapping
fn swap_vectors(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    std::mem::swap(a, b); // Built-in swap function
}

// Or manual approach
fn manual_swap() {
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];

    let temp = a; // Move a to temp
    a = b; // Move b to a
    b = temp; // Move temp to b

    println!("a: {:?}, b: {:?}", a, b);
}

pub fn test() {
    process_data();
    // Rust - Ownership is transferred
    let original = vec![1, 2, 3];
    let moved = original; // Ownership transferred
    // println!("{:?}", original);  // ❌ Error: original no longer owns the data
    println!("{:?}", moved); // ✅ Works: moved now owns the data

    // Copy types (like C# value types) - copied, not moved
    let x = 5; // i32 implements Copy
    let y = x; // x is copied to y
    println!("{}, {}", x, y); // ✅ Works: x is still valid

    // Move types (like C# reference types) - moved, not copied
    let s1 = String::from("hello"); // String doesn't implement Copy
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1);           // ❌ Error: s1 is no longer valid
}
// Rust - borrowing with & and &mut
fn modify_value(value: &mut i32) {
    // Mutable borrow
    *value += 10;
}
#[allow(unused)]
fn read_value(value: &i32) {
    // Immutable borrow
    println!("{}", value);
}
#[allow(unused)]
fn main() {
    let mut x = 5;

    read_value(&x); // Borrow immutably
    modify_value(&mut x); // Borrow mutably

    println!("{}", x); // x is still owned here
}

// 1. Move after use
fn problem_1() {
    let name = String::from("Alice");
    let greeting = format!("Hello, {name}!");
    let upper = name.to_uppercase(); // hint: borrow instead of move
    println!("{greeting} — {upper}");
}

// 2. Mutable + immutable borrow overlap
fn problem_2() {
    let mut numbers = vec![1, 2, 3];
    let first = &numbers[0];
    // numbers.push(4);            // hint: reorder operations
    println!("first = {first}");
}

// 3. Returning a reference to a local
fn problem_3() -> String {
    let s = String::from("hello");
    s // hint: return owned value, not &str
}

// 1. format! already borrows — the fix is that format! takes a reference.
//    The original code actually compiles! But if we had `let greeting = name;`
//    then fix by using &name:
fn solution_1() {
    let name = String::from("Alice");
    let greeting = format!("Hello, {}!", &name); // borrow
    let upper = name.to_uppercase(); // name still valid
    println!("{greeting} — {upper}");
}

// 2. Use the immutable borrow before the mutable operation:
fn solution_2() {
    let mut numbers = vec![1, 2, 3];
    let first = numbers[0]; // copy the i32 value (i32 is Copy)
    numbers.push(4);
    println!("first = {first}");
}

// 3. Return the owned String (already correct — common beginner confusion):
fn solution_3() -> String {
    let s = String::from("hello");
    s // ownership transferred to caller — this is the correct pattern
}
