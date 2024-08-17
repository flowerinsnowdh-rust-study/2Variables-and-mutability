fn main() {
    mutability();
    constants();
    shadowing();
}

fn mutability() {
    println!("1. Mutability");
    let x = 5;
    println!("x = {x}");
    /*
    error[E0384]: cannot assign twice to immutable variable `x`
    x = 6; // ❌
    println!("x = {x}");
     */

    let mut y = 7;
    y = 8; // ✔
    println!("y = {y}");
}

fn constants() {
    println!("2. Constants");
    const PI : f32 = 3.14;
    println!("PI = {PI}");
}

fn shadowing() {
    println!("3. Shadowing");
    let x = 5;
    let x = x + 1;
    {
        let x = 7;
        println!("inner x = {x}")
    }
    println!("outer x = {x}");

    let spaces = "    ";
    /*
    error[E0308]: mismatched types
    spaces = spaces.len(); // ❌
     */
    let spaces = spaces.len(); // ✔
    println!("spaces = {spaces}")
}