fn main() {
    // Variables in Rust are immutable by default.
    // You can however make a variable mutable by adding the 'mut' keyword.
    // You are not required to type let statements when the compiler can figure out the type
    // similar to typescript .. Below we type it anyway.
    let missiles: i32 = 8;
    let ready: i32 = 2;
    // Another even stricter variable declaration is the constant.
    // Typing is required on constants. and Screaming SnakeCase used by convention.
    // Constants can be placed outside functions in the global scope, available program wide.
    // They're also 'in-lined' at compile time adding to the speed of Rust.
    const WARP_FACTOR: f64 = 9.98;
    println!(
        "Firing {} of my {} missiles (which travel at {} warp)!",
        ready, missiles, WARP_FACTOR,
    );
}
