// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::{print_difference, on_off, print_array, ding, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // 1. Pass parts of `coords` to the `print_difference` function. This should show the difference
    // between the two numbers in coords when you do `cargo run`.  Use tuple indexing.
    //
    // The `print_difference` function is defined below the `main` function. It may help if you look
    // at how it is defined.
    //
    print_difference( coords.0, coords.1 );


    // 2. We want to use the `print_array` function to print coords...but coords isn't an array!
    // Create an array of type [f32; 2] and initialize it to contain the
    // information from coords.  Uncomment the print_array line and run the code.
    //
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);


    let series = [1, 1, 2, 3, 5, 8, 13];
    // 3. Make the `ding` function happy by passing it the value 13 out of the `series` array.
    // Use array indexing.  Done correctly, `cargo run` will produce the additional output
    // "Ding, you found 13!"
    //
    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    //
    on_off(mess.2[1].0);

    // 5.  What a mess -- functions in a binary! Let's get organized!
    //
    // - Make a library file (src/lib.rs)
    // - Move all the functions (except main) into the library
    // - Make all the functions public with `pub`
    // - Bring all the functions into scope using use statements. Remember, the name of the library
    //   is defined in Cargo.toml.  You'll need to know that to `use` it.
    //
    // `cargo run` should produce the same output, only now the code is more organized. 🎉

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.
    print_distance(coords);
}
