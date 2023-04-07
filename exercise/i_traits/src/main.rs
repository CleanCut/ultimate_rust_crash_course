// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code)]

// 1a: Create a `Colorful` trait with a single `color` method which takes an immutable reference to
// self and returns a String describing that item's color.
//
// You must also complete 1b before the code will compile.

// trait Colorful ...

// 1b. Implement the `Colorful` trait for the `Hat` struct:
//
// - The `colorful` method of the `Colorful` trait should return the following String values:
//   - "red" if the size is 0 through 5
//   - "green" - if the size is 6 or 7
//   - "blue" - for any other size
//
// The code should compile once 1b is complete, but there will be no output until 1c is completed.

struct Hat {
    size: i32,
}

// impl Colorful for Hat ...

fn main() {
    // 1c. Uncomment and run the code below. If you correctly implemented Colorful for Hat, then
    // the order of the colors in the output will be red, green, and blue.

    // let small_hat = Hat { size: 2 };
    // let medium_hat = Hat { size: 7 };
    // let large_hat = Hat { size: 100 };
    // describe_three_hats(small_hat, medium_hat, large_hat);

    // 2. Implement the Colorful trait for the type i32. Then uncomment and run the code below.
    //
    // The `colorful` method for an i32 should return the String values:
    //   - "orange" - If the number is even (see hint)
    //   - "purple" - If the number is odd
    //
    // Hint: You may want to use the `is_even` function (see the bottom of this file).

    // for i in range(0..=3) {
    //     println!("{} is {}", i, i.color());
    // }

    // 3: Write a generic function named `fortune` that takes anything that implements the
    // Colorful trait and prints out the color in some message (for example: "The color I see in
    // your future is ..."). Then uncomment and run the code below.

    // fortune(small_hat);
    // fortune(2);

    // Challenge 2: Create a trait named `EvenOdd` with a method `is_even` that takes an immutable
    // reference to self and returns whether or not self is even. Implement EvenOdd for i32. Then
    // refactor the `colorful` method for `i32` to use the is_even method.
}

// A helpful function
fn describe_three_hats(hat1: &Hat, hat2: &Hat, hat3: &Hat) {
    for hat in [hat1, hat2, hat3] {
        let largeness = if hat.size < 3 {
            "small"
        } else if hat.size < 9 {
            "medium"
        } else {
            "large"
        };
        println!("The {} hat is {}", largeness, hat.color());
        println!("The {} hat is {}", largeness, hat.color());
        println!("The {} hat is {}", largeness, hat.color());
    }
}

// NOTE: A helpful function to check if a number is even.
fn is_even(number: i32) -> bool {
    number % 2 == 0
}
