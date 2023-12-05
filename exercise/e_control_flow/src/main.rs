// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // 1. Use an unconditional `loop` to count how many times we can double `bunnies` until there
    // are over 500 bunnies. (Hint: The answer is 8 times)
    //
    // Inside the loop:
    // - Add 1 to `count`
    // - Multiply `bunnies` by 2
    // - If `bunnies` is larger than 500, break out of the loop.

    let mut count = 0;
    let mut bunnies = 2;

    // (write your `loop` here)

    println!(
        "Bunnies doubled {} times before there were more than 500",
        count
    );

    // 2. Use a `for` loop to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (add each value to the `sum` variable).  Hint: You should get 255

    let mut sum = 0;

    // (write the `for` loop here)

    println!("The sum is {}", sum);

    // 3. Use a `while` loop to add 12 numbers to the `fives` vector.
    //
    // The loop should continue while `fives.len()` is less than 12.
    //
    // Each time through the loop:
    // - Call `fives.push(number)` to push (a copy of) `number` onto the vector
    // - Add 5 to `number`
    //
    // If you do this correctly, the vector will be [5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60]

    let mut fives: Vec<i32> = vec![];
    let mut number = 5;

    // (write the `while` loop here)

    println!("Here are the first 12 multiples of 5: {:?}", fives);

    // 4. Use `if`, `else if` and `else` inside the `for` loop below to do the following:
    //
    // - If the number is 0, then add 7 to `total`
    // - If the number is 1 or 2 then add 30 to `total`
    // - If the number is anything else, subtract 5 from `total`
    //
    // Hint: The total should be 52

    let mut total = 0;
    let numbers = vec![0, 1, 2, 3, 4, 5];
    for number in numbers {
        // (write your `if/else` expression here)
    }

    println!("The total is {}", total);

    // Challenge: Change the implementation of your answers to #1-#3 as follows:
    //
    // - Change #1 to use `while`
    // - Change #2 to use `loop`
    // - Change #3 to use `for` and a range (multiply the range value by 5 inside your loop before
}
