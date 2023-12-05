// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f64, f64) = (6.3, 15.0);
    // 1. Pass the two elements of the `coords` tuple as two separate arguments to the
    // `print_difference` function. Use tuple indexing.
    //
    // The `print_difference` function is defined near the end of this file if you would like to
    // look at how it is defined.
    //
    //print_difference( ... );   // Uncomment and finish this line

    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f64; 2] and initialize it to contain the
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

    // 5. (Part A)
    //
    // Uncomment the line below, run the code, and examine the output. Then go refactor the
    // `print_distance` function according to the instructions in the comments inside that function.

    // print_distance(coords);
}

// 5. (Part B)
//
// Using `z.0` and `z.1` is not nearly as nice as using `x` and `y`.  Lucky for us, Rust supports
// destructuring function arguments.  Try replacing `z` in the parameter list below with `(x, y)`
// and then adjust the function body below to use `x` and `y` instead of `z.0` and `z.1`
//
// You should be able to run the code again and get the output as before.
fn print_distance(z: (f64, f64)) {
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt()
    );
}

// Challenge:
//
// Although types can often be inferred by the compiler, sometimes we write them out for clarity.
// Like we did with the `let coords: (f64, f64) = ...` declaration at the top of the `main`
// function.
//
// - Add the type annotation for the `series` variable in `main`.
// - Add the type annotation for the `mess` variable in `main`. (This may be a good example of why
//   it is nice to *not* have to add the type annotation! 😆)

fn print_difference(x: f64, y: f64) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f64; 2]) {
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
