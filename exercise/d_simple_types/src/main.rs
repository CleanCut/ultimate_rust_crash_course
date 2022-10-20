// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    //print_difference( ... );   // Uncomment and finish this line

    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    //let coords_arr...               // create an array literal out of parts of `coord` here
    //print_array(coords_arr);        // and pass it in here (this line doesn't need to change)

    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    //ding(...);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    //on_off(mess.2 ...);

    // Challenge 1 (part A)
    //
    // Uncomment the line below, run the code, and examine the output. Then go refactor the
    // `print_distance` function according to the instructions in the comments inside that function.

    // print_distance(coords);
}

// Challenge 2
//
// Reorganize the functions into a library.
//
// - Create the root library file
// - Change the variables `coords`, `series` and `mess` to be constants and move them to the library
// - Move the `ding` and `on_off` functions into the root (top level) of the library.
// - Create a `print` submodule and move the functions that start with `print_*` into it.
// - Remove the `print_` prefix from the functions in the `print` submodule.
// - Remember to access the functions through their parent modules like this: `print::array(...)`

fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

fn print_distance(z: (f32, f32)) {
    // Challenge 1 (part B)
    //
    // Using `z.0` and `z.1` is not nearly as nice as using `x` and `y`.  Lucky for us, Rust
    // supports destructuring function arguments.  Try replacing `z` in the parameter list above
    // with `(x, y)` and then adjust the function body below to use `x` and `y` instead of `z.0` and
    // `z.1`
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}
