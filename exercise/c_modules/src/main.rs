// 1. Organize code into a library without changing the output of the program.
//
// For each step of this exercise, you should be able to run the program before and after your
// change without affecting the output of the program.
//
// Move the constants below (FIRST, SECOND, and THIRD) into the library:
// - Create a `src/lib.rs` file
// - Move all of the constants into lib.rs
// - Make the constants public by adding the `pub` keyword in front of them
// - Add `use` statement(s) to main.rs to bring the constants into scope.
//
// Hint: the name of the library is defined in Cargo.toml

const FIRST: i32 = 1;
const SECOND: i32 = 2;
const THIRD: i32 = 3;

// 2. Create a library module named `sound` and move the animal functions into it.
//
// - In your lib.rs file add the line `pub mod sound;`
// - Create a `src/sound.rs` file for your module
// - Move the `dog`, `cat`, and `fox` functions into sound.rs
// - Make the functions public by adding the `pub` keyword in front of them
// - Add a `use` statement to bring the `sound` module into scope.
// - Change the function calls to access the functions through the `sound` module.
//   For example: sound::dog()

fn dog() {
    println!("Dog goes WOOF!");
}

fn cat() {
    println!("Cat goes MEOW!");
}

fn fox() {
    println!("What does the fox say???");
}

fn main() {
    print!("Listening to animal {}: ", FIRST);
    dog();

    print!("Listening to animal {}: ", SECOND);
    cat();

    print!("Listening to animal {}: ", THIRD);
    fox();
}

// Challenge 1
//
// - Move the `dog` and `cat` functions into a submodule `animal::sound::tame`
// - Move the `fox` function into a submodule `animal::sound::wild`
//
// Hint: You will need to create a subdirectory for the top-level `sound` modules' submodules to
// be placed in.

// Challenge 2
//
// Create an `animal::prelude` module which re-exports all of the constants and functions of the
// library. (A real library would only re-export the most commonly-used items in its prelude.)
//
// Change your `use` statement(s) in main.rs to just `use animal::prelude::*`
//
// Hint: You will need `pub use` to re-export an item, for more details see:
// https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
