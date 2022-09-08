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

    // You can destructure variables
    let (silly, dumb, dooda) = ("LOL", "snakes", "sherpa");

    // Variable Scope
    // Scoping is block by default, and apparently a lot of the rules are the same to
    // ES6+ javascript. For example:
    fn scopeEx() {
        let x = 99;
        {
            let y = 88;
            println!("{} is less than {}", y, x); // 88 is less than 99
        }
        // println!("{} is less than {}", y, x); // ERROR (y is dropped from scope immediately leaving block)
    }

    // Variable Shadowing
    // This is likely a Rust concept that looks a lot like what you see in
    // javascript for instance, but they make a point to call it 'shadowing'.
    // But it's the way to accomplish reassignment, etc.
    let d = 10;
    {
        let d = 50;
        // In this block d will be 50
    }
    // In the outter scope d will be 10

    let mut doggie = "woof!";
    let doggie = "bark!";
    // doggie will now be immutable 'bark!'

    let meme = "More Cowbell!!";
    let meme = 99;
    // We just changed the type of the variable with 'shadowing'
}
