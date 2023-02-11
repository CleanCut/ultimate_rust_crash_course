// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_assignments)]

// Q. What's the difference between a string literal and a borrowed string slice?
//
// A. A string literal is what is written in your source code. e.g. "this is a string literal",
// while a borrowed strings slice (&str) is the *type* of the string literal. So:
//
//   let my_name: &str = "Nathan";
//
// The variable my_name is a borrowed string slice, initialized by the string literal "Nathan".

fn main() {
    // 1. Using unicode escape codes, use println to print out a sparkles emoji (codepoint 2728).

    // println!( ... );

    // 2. Uncomment the commented line below and set the value of `favorite` to the emoji "🍓"
    // (codepoint 1f353).
    //
    // - You can type (or copy-and-paste) in the strawberry emoji, or use unicode escape codes.
    // - Use .to_string() to convert the string literal into a String.

    let mut favorite = String::new();
    // favorite = ...
    if favorite != "" {
        println!("Everyone's favorite fruit is: {favorite}");
    }

    // 3. Uncomment the code below. Using newline escape codes, complete the string literal so it
    // prints out:
    //
    // Now is
    // the time
    // for all
    // great men

    // let saying = "Now ...
    // println!("{saying}");

    // Challenge: Change the string literal in #3 so that it:
    //
    // - Is a multiline string (uses real newlines instead of newline escape codes)
    // - Is indented properly with the code
    // - Still outputs the exact same text, without any leading spaces
}
