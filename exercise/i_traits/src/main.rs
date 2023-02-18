#![allow(dead_code)]

// Items which implement the `Colorful` trait have a `color` method which will return a string
// describing that item's color.
trait Colorful {
    fn color(self: &Self) -> String;
}

// 1. Define a struct `Hat` with one field `size` that is a `u8`.

// struct Hat ...

// 2a. Hats are colorful! Implement the `Colorful` trait which returns:
//
// - Red - (as a String) if the size is 0 through 5
// - Green - if the size is 6 or 7
// - Blue - for any other size

// impl Colorful for Hat ...

fn main() {
    // 2b. Uncomment and run the code below. The hats should be red, green, and blue--in that order.

    // let small_hat = Hat { size: 2 };
    // let medium_hat = Hat { size: 7 };
    // let large_hat = Hat { size: 100 };
    // println!("The small hat is {}", small_hat.color());
    // println!("The medium hat is {}", medium.color());
    // println!("The large hat is {}", large_hat.color());

    // 3. Implement the Colorful trait for the type i32. Then uncomment and run the code below.
    //
    // - Orange - If the number is even*
    // - Purple - If the number is odd
    //
    // *You may use the `is_even` function from the bottom of this file if you like.

    // for i in range(0..=3) {
    //     println!("{} is {}", i, i.color());
    // }

    // 4. Implement an associated function named `new` for the `Hat` struct which takes
    // an u8 as an argument and returns a Hat of that size.
    //
    // Change the code in 2b to use Hat::new( ... ) to create the hats.
    //
    // Note: This has nothing to do with the Colorful trait.

    // Challenge 1: Write a generic function named `fortune` that takes anything that implements the
    // Colorful trait and prints out the color in some message (for example: "The color I see in
    // your future is ..."). Then uncomment and run the code below.

    // fortune(small_hat);
    // fortune(2);

    // Challenge 2: Create a trait `
}

// Note: Here is how to check if a number is even.
fn is_even(number: i32) -> bool {
    number % 2 == 0
}
